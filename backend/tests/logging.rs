mod support;

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use oph_trainer_backend::practice::PracticeService;
use serde_json::{Value, json};
use tower::ServiceExt;
use tracing::instrument::WithSubscriber;

#[derive(Clone, Default)]
struct LogBuffer(Arc<Mutex<Vec<u8>>>);

impl Write for LogBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    key: &str,
    body: Value,
) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    if !key.is_empty() {
        request = request.header("authorization", format!("Bearer {key}"));
    }
    let response = app
        .clone()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn activity_logs_include_request_context_without_credentials_or_learner_text() {
    let app = support::app(PracticeService::new(support::bank(3))).await;
    let empty_bank = support::app(PracticeService::new(support::bank(0))).await;
    let logs = LogBuffer::default();
    let writer = logs.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("oph_trainer_backend=debug")
        .with_ansi(false)
        .without_time()
        .with_writer(move || writer.clone())
        .finish();

    let key = async {
        let (status, account) = send(&app, "POST", "/accounts", "", json!({})).await;
        assert_eq!(status, StatusCode::OK);
        let key = account["account_key"].as_str().unwrap().to_owned();
        let (status, quiz) = send(
            &app,
            "POST",
            "/quizzes?private-query-value",
            &key,
            json!({}),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(quiz["questions"].as_array().unwrap().len(), 3);
        for question_id in ["q0", "q1"] {
            let (status, _) = send(
                &app,
                "POST",
                "/answers",
                &key,
                json!({
                    "question_id": question_id,
                    "answer": "private-learner-answer"
                }),
            )
            .await;
            assert_eq!(status, StatusCode::OK);
        }
        let completed = json!({
            "quiz_id": "private-quiz-id",
            "question_ids": ["q0"],
            "outcomes": [{"question_id": "q0", "correct": true}]
        });
        for _ in 0..2 {
            assert_eq!(
                send(&app, "POST", "/progress/quizzes", &key, completed.clone())
                    .await
                    .0,
                StatusCode::NO_CONTENT
            );
        }
        assert_eq!(
            send(&app, "GET", "/progress", &key, Value::Null).await.0,
            StatusCode::OK
        );
        assert_eq!(
            send(&app, "DELETE", "/progress", &key, Value::Null).await.0,
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            send(
                &app,
                "POST",
                "/quizzes",
                &key,
                json!({"topic_ids": ["private-topic-id"]})
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            send(
                &app,
                "POST",
                "/answers",
                &key,
                json!({"question_id": "private-question-id", "answer": "private-learner-answer"})
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            send(&app, "GET", &format!("/{key}"), &key, Value::Null)
                .await
                .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            send(
                &app,
                "GET",
                "/progress",
                "private-invalid-credential",
                Value::Null
            )
            .await
            .0,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            send(
                &app,
                "POST",
                "/answers",
                &key,
                json!({"answer": "private-learner-answer"})
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            send(&empty_bank, "POST", "/quizzes", "", json!({})).await.0,
            StatusCode::SERVICE_UNAVAILABLE
        );
        key
    }
    .with_subscriber(subscriber)
    .await;

    let output = String::from_utf8(logs.0.lock().unwrap().clone()).unwrap();
    for expected in [
        "Account created",
        "Quiz selected",
        "Answer assessed",
        "Completed quiz saved",
        "already_saved=true",
        "Progress loaded",
        "Progress reset",
        "reason=\"not_configured\"",
        "request_id=",
        "method=POST",
        "route=\"/answers\"",
        "route=\"unmatched\"",
        "status=200",
        "status=204",
        "status=400",
        "status=401",
        "status=404",
        "status=422",
        "status=503",
        "elapsed_ms=",
        "InvalidTopics",
        "UnknownQuestion",
    ] {
        assert!(
            output.contains(expected),
            "Missing {expected} in logs:\n{output}"
        );
    }
    for secret in [
        key.as_str(),
        "private-query-value",
        "private-learner-answer",
        "private-quiz-id",
        "private-topic-id",
        "private-question-id",
        "private-invalid-credential",
        "Bearer",
    ] {
        assert!(!output.contains(secret), "Logs exposed {secret}:\n{output}");
    }
    let completions: Vec<_> = output
        .lines()
        .filter(|line| line.contains("Request completed"))
        .collect();
    assert_eq!(completions.len(), 14);
    assert!(
        completions
            .iter()
            .any(|line| line.contains("WARN") && line.contains("status=404"))
    );
    assert!(
        completions
            .iter()
            .any(|line| line.contains("ERROR") && line.contains("status=503"))
    );
    assert!(output.lines().all(|line| line.contains("request_id=")));
}

#[tokio::test]
async fn warnings_retain_request_context_with_a_quiet_filter() {
    let app = support::app(PracticeService::new(support::bank(3))).await;
    let logs = LogBuffer::default();
    let writer = logs.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("warn")
        .with_ansi(false)
        .without_time()
        .with_writer(move || writer.clone())
        .finish();

    let (status, _) = send(&app, "GET", "/progress", "", Value::Null)
        .with_subscriber(subscriber)
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let output = String::from_utf8(logs.0.lock().unwrap().clone()).unwrap();
    for expected in [
        "WARN",
        "request_id=",
        "method=GET",
        "route=\"/progress\"",
        "status=401",
    ] {
        assert!(
            output.contains(expected),
            "Missing {expected} in logs:\n{output}"
        );
    }
}

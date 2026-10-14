mod support;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use oph_trainer_backend::{
    accounts::AccountService, grading::GradingService, practice::PracticeService,
    question_bank::AnswerCriteria, routes,
};
use serde_json::{Value, json};
use tokio::{net::TcpListener, task::JoinHandle};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct ProviderState {
    calls: Arc<Mutex<Vec<(HeaderMap, Value)>>>,
}

struct Provider {
    endpoint: String,
    state: ProviderState,
    task: JoinHandle<()>,
}

impl Drop for Provider {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn decisions(
    State(state): State<ProviderState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Response {
    let answer = payload["state"]["learner_answer"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut decisions: serde_json::Map<String, Value> = payload["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|id| {
            let choice = if id.starts_with("required_") {
                "covered"
            } else {
                "not_asserted"
            };
            (
                id.clone(),
                json!({ "type": "choice", "choice": choice, "confidence": 0.8 }),
            )
        })
        .collect();
    state.calls.lock().unwrap().push((headers, payload));
    match answer.as_str() {
        "error" => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        "redirect" => {
            return (
                StatusCode::TEMPORARY_REDIRECT,
                [("location", "/api/alpha/decisions")],
            )
                .into_response();
        }
        "invalid" => {
            decisions.get_mut("contradiction_1").unwrap()["choice"] = json!("maybe");
        }
        "wrong type" => {
            decisions.get_mut("contradiction_1").unwrap()["type"] = json!("noul");
        }
        "incomplete" => {
            decisions.remove("contradiction_1");
        }
        "extra" => {
            decisions.insert(
                "unexpected".into(),
                json!({"type": "choice", "choice": "covered"}),
            );
        }
        "low required confidence" => {
            decisions.get_mut("required_1").unwrap()["confidence"] = json!(0.799);
        }
        "low contradiction confidence" => {
            decisions.get_mut("contradiction_1").unwrap()["confidence"] = json!(0.799);
        }
        "high confidence" => {
            for decision in decisions.values_mut() {
                decision["confidence"] = json!(0.95);
            }
        }
        "missing confidence" => {
            decisions
                .get_mut("required_1")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove("confidence");
        }
        "invalid confidence" => {
            decisions.get_mut("contradiction_1").unwrap()["confidence"] = json!("0.9");
        }
        "out of range confidence" => {
            decisions.get_mut("required_1").unwrap()["confidence"] = json!(1.1);
        }
        "malformed" => return "not json".into_response(),
        "timeout" => tokio::time::sleep(Duration::from_secs(9)).await,
        "oversized response" => return "x".repeat(70_000).into_response(),
        _ => {}
    }
    if answer == "incorrect" {
        decisions.get_mut("required_1").unwrap()["choice"] = json!("missing");
    }
    if answer == "contradiction" {
        decisions.get_mut("contradiction_1").unwrap()["choice"] = json!("asserted");
    }
    if answer == "both" {
        decisions.get_mut("required_0").unwrap()["choice"] = json!("missing");
        decisions.get_mut("contradiction_0").unwrap()["choice"] = json!("asserted");
    }
    Json(json!({"answers": decisions})).into_response()
}

impl Provider {
    async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!(
            "http://{}/api/alpha/decisions",
            listener.local_addr().unwrap()
        );
        let state = ProviderState::default();
        let app = Router::new()
            .route("/api/alpha/decisions", post(decisions))
            .with_state(state.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            endpoint,
            state,
            task,
        }
    }

    fn service(&self) -> GradingService {
        GradingService::with_provider_endpoint(
            Some("test-provider-secret".into()),
            self.endpoint.clone(),
        )
        .unwrap()
    }

    fn calls(&self) -> usize {
        self.state.calls.lock().unwrap().len()
    }
}

async fn app(grading: GradingService, use_x_forwarded_for: bool) -> Router {
    let mut bank = support::bank(3);
    bank.questions[1].grading = AnswerCriteria::Explanation {
        required_points: vec!["Explain why".into(), "Second required point".into()],
        optional_points: vec!["Extra detail".into()],
        contradictions: vec!["A false claim".into(), "Another false claim".into()],
    };
    routes::api_with_grading(
        PracticeService::new(bank),
        AccountService::new(support::database("sqlite::memory:").await),
        grading,
        use_x_forwarded_for,
    )
}

async fn send(app: Router, body: &str, peer: &str, forwarded: Option<&str>) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .method("POST")
        .uri("/answers")
        .header("Authorization", "Bearer learner-secret");
    request = request.header("content-type", "application/json");
    if let Some(forwarded) = forwarded {
        request = request
            .header("x-forwarded-for", forwarded)
            .header("forwarded", "for=198.51.100.99")
            .header("x-real-ip", "198.51.100.99");
    }
    let mut request = request.body(Body::from(body.to_owned())).unwrap();
    request
        .extensions_mut()
        .insert(ConnectInfo(peer.parse::<SocketAddr>().unwrap()));
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    assert_eq!(response.headers()["cache-control"], "no-store");
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8(bytes.to_vec()).unwrap())),
    )
}

async fn answer(app: &Router, text: &str, peer: &str, forwarded: Option<&str>) -> Value {
    let (status, feedback) = send(
        app.clone(),
        &json!({"question_id": "q1", "answer": text}).to_string(),
        peer,
        forwarded,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(feedback["reference_answer"], "Authored reference");
    assert_eq!(feedback["source_url"], "https://example.com/syllabus");
    if feedback["status"] == "needs_self_assessment" {
        assert!(feedback.get("explanation_feedback").is_none());
    } else {
        assert!(feedback["explanation_feedback"].is_object());
    }
    feedback["status"].clone()
}

#[tokio::test]
async fn validation_and_short_answers_never_reach_the_provider() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    for (length, expected) in [
        (4_001, StatusCode::BAD_REQUEST),
        (20_000, StatusCode::PAYLOAD_TOO_LARGE),
    ] {
        let body = json!({"question_id": "q1", "answer": "x".repeat(length)}).to_string();
        assert_eq!(
            send(app.clone(), &body, "192.0.2.1:1234", None).await.0,
            expected
        );
    }
    for (text, expected) in [("  ALIAS  ", "correct"), ("unaccepted", "incorrect")] {
        let (status, feedback) = send(
            app.clone(),
            &json!({"question_id":"q0", "answer":text}).to_string(),
            "192.0.2.1:1234",
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(feedback["status"], expected);
    }
    assert_eq!(provider.calls(), 0);
    assert_eq!(
        answer(&app, "good", "192.0.2.1:1234", None).await,
        "correct"
    );
    assert_eq!(provider.calls(), 1);
}

#[tokio::test]
async fn provider_failures_and_missing_credentials_leave_answers_ungraded() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    for text in [
        "error",
        "invalid",
        "wrong type",
        "malformed",
        "oversized response",
        "redirect",
        "incomplete",
        "extra",
    ] {
        assert_eq!(
            answer(&app, text, "192.0.2.1:1234", None).await,
            "needs_self_assessment"
        );
    }
    assert_eq!(provider.calls(), 8);
    let unavailable = support::app(PracticeService::new(support::bank(3))).await;
    assert_eq!(
        answer(&unavailable, "good", "192.0.2.1:1234", None).await,
        "needs_self_assessment"
    );
}

#[tokio::test]
async fn every_decision_needs_at_least_point_eight_confidence_to_grade() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    for (text, expected) in [
        ("good", "correct"),
        ("incorrect", "incorrect"),
        ("contradiction", "incorrect"),
        ("high confidence", "correct"),
        ("low required confidence", "needs_self_assessment"),
        ("low contradiction confidence", "needs_self_assessment"),
        ("missing confidence", "needs_self_assessment"),
        ("invalid confidence", "needs_self_assessment"),
        ("out of range confidence", "needs_self_assessment"),
    ] {
        assert_eq!(
            answer(&app, text, "192.0.2.1:1234", None).await,
            expected,
            "{text}"
        );
    }
}

#[tokio::test]
async fn failed_attempts_count_toward_client_limits_despite_spoofed_headers() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    for _ in 0..60 {
        assert_eq!(
            answer(&app, "error", "192.0.2.1:1234", None).await,
            "needs_self_assessment"
        );
    }
    assert_eq!(
        answer(&app, "good", "192.0.2.1:9999", Some("198.51.100.2")).await,
        "needs_self_assessment"
    );
    assert_eq!(provider.calls(), 60);
    assert_eq!(
        answer(&app, "good", "192.0.2.2:1234", None).await,
        "correct"
    );
    assert_eq!(provider.calls(), 61);
}

#[tokio::test]
async fn forwarded_ip_mode_separates_clients_and_falls_back_to_the_socket_ip() {
    let provider = Provider::new().await;
    let app = app(provider.service(), true).await;
    for _ in 0..60 {
        assert_eq!(
            answer(
                &app,
                "good",
                "192.0.2.1:1234",
                Some(" 2001:db8::1 , 192.0.2.1"),
            )
            .await,
            "correct"
        );
    }
    assert_eq!(
        answer(
            &app,
            "good",
            "192.0.2.2:9999",
            Some("2001:db8::1, 192.0.2.2"),
        )
        .await,
        "needs_self_assessment"
    );
    assert_eq!(provider.calls(), 60);
    assert_eq!(
        answer(&app, "good", "192.0.2.1:1234", Some("198.51.100.2")).await,
        "correct"
    );

    for _ in 0..60 {
        assert_eq!(
            answer(&app, "good", "192.0.2.1:1234", None).await,
            "correct"
        );
    }
    for forwarded in [None, Some("garbage, 198.51.100.3"), Some("")] {
        assert_eq!(
            answer(&app, "good", "192.0.2.1:9999", forwarded).await,
            "needs_self_assessment"
        );
    }
    assert_eq!(provider.calls(), 121);
}

#[tokio::test]
async fn provider_timeout_leaves_the_answer_ungraded() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    assert_eq!(
        answer(&app, "timeout", "192.0.2.1:1234", None).await,
        "needs_self_assessment"
    );
    assert_eq!(provider.calls(), 1);
}

#[tokio::test]
async fn individual_decisions_determine_the_grade_and_return_only_authored_feedback() {
    let provider = Provider::new().await;
    let app = app(provider.service(), false).await;
    for (text, status, covered, contradictions) in [
        ("reworded learner answer", "correct", [true, true], vec![]),
        ("incorrect", "incorrect", [true, false], vec![]),
        (
            "contradiction",
            "incorrect",
            [true, true],
            vec!["Another false claim"],
        ),
        ("both", "incorrect", [false, true], vec!["A false claim"]),
    ] {
        let (response_status, feedback) = send(
            app.clone(),
            &json!({"question_id":"q1", "answer":text}).to_string(),
            "192.0.2.1:1234",
            None,
        )
        .await;
        assert_eq!(response_status, StatusCode::OK);
        assert_eq!(
            feedback,
            json!({
                "question_id": "q1", "status": status, "reference_answer": "Authored reference", "source_url": "https://example.com/syllabus",
                "explanation_feedback": { "required_points": [{"point": "Explain why", "covered": covered[0]}, {"point": "Second required point", "covered": covered[1]}], "contradictions": contradictions }
            })
        );
    }
    assert_eq!(provider.calls(), 4);
    let calls = provider.state.calls.lock().unwrap();
    let (headers, payload) = &calls[0];
    assert_eq!(headers["authorization"], "Bearer test-provider-secret");
    assert_eq!(payload["model"], "typesafe/jev-1.13");
    assert_eq!(payload["provider"]["allow_fallbacks"], false);
    assert_eq!(
        payload["state"],
        json!({"prompt":"Question 1", "reference_answer":"Authored reference", "required_points":["Explain why", "Second required point"], "optional_points":["Extra detail"], "contradictions":["A false claim", "Another false claim"], "learner_answer":"reworded learner answer"})
    );
    let questions = payload["questions"].as_object().unwrap();
    assert_eq!(questions.len(), 4);
    for index in 0..2 {
        let required = &questions[&format!("required_{index}")];
        assert_eq!(required["type"], "choice");
        assert!(required["criteria"].get("covered").is_some());
        assert!(required["criteria"].get("missing").is_some());
        let contradiction = &questions[&format!("contradiction_{index}")];
        assert_eq!(contradiction["type"], "choice");
        assert!(contradiction["criteria"].get("asserted").is_some());
        assert!(contradiction["criteria"].get("not_asserted").is_some());
    }
    assert!(!payload.to_string().contains("secret"));
    assert!(!payload.to_string().contains("learner-secret"));
}

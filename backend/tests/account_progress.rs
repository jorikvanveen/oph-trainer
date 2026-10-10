mod support;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Method, Request, StatusCode, header},
};
use migration::SchemaManager;
use oph_trainer_backend::{
    accounts::AccountService, entity::question_outcomes as question_outcome,
    practice::PracticeService, routes,
};
use sea_orm::sea_query::Index;
use serde_json::{Value, json};
use tower::ServiceExt;

struct Reply {
    status: StatusCode,
    headers: HeaderMap,
    body: Value,
}

async fn request(
    app: &Router,
    method: Method,
    path: &str,
    account_key: Option<&str>,
    body: Value,
) -> Reply {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(account_key) = account_key {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {account_key}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body = if bytes.is_empty() {
        Value::Null
    } else if headers[header::CONTENT_TYPE] == "application/json" {
        serde_json::from_slice(&bytes).unwrap()
    } else {
        Value::String(String::from_utf8(bytes.to_vec()).unwrap())
    };
    Reply {
        status,
        headers,
        body,
    }
}

async fn create_account(app: &Router) -> String {
    let reply = request(app, Method::POST, "/accounts", None, Value::Null).await;
    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.headers[header::CACHE_CONTROL], "no-store");
    assert!(!reply.headers.contains_key(header::SET_COOKIE));
    let key = reply.body["account_key"].as_str().unwrap().to_owned();
    assert_eq!(key.len(), 64);
    key
}

fn quiz(id: &str, outcomes: &[(&str, bool)]) -> Value {
    json!({
        "quiz_id": id,
        "question_ids": outcomes.iter().map(|(id, _)| id).collect::<Vec<_>>(),
        "outcomes": outcomes.iter().map(|(id, correct)| json!({"question_id": id, "correct": correct})).collect::<Vec<_>>()
    })
}

async fn commit(app: &Router, account_key: &str, quiz: Value) {
    let reply = request(
        app,
        Method::POST,
        "/progress/quizzes",
        Some(account_key),
        quiz,
    )
    .await;
    assert_eq!(reply.status, StatusCode::NO_CONTENT, "{}", reply.body);
}

async fn progress(app: &Router, account_key: &str) -> Value {
    let reply = request(
        app,
        Method::GET,
        "/progress",
        Some(account_key),
        Value::Null,
    )
    .await;
    assert_eq!(reply.status, StatusCode::OK, "{}", reply.body);
    assert_eq!(reply.headers[header::CACHE_CONTROL], "no-store");
    reply.body
}

#[tokio::test]
async fn practice_creates_an_account_key_and_abandoned_answers_leave_no_history() {
    let app = support::app(PracticeService::new(support::bank(4))).await;
    let reply = request(&app, Method::POST, "/quizzes", None, json!({})).await;
    assert_eq!(reply.status, StatusCode::OK);
    let key = reply.body["account_key"].as_str().unwrap();
    assert_eq!(key.len(), 64);
    assert!(!reply.headers.contains_key(header::SET_COOKIE));
    let answer = request(
        &app,
        Method::POST,
        "/answers",
        Some(key),
        json!({"question_id":"q0","answer":"accepted answer"}),
    )
    .await;
    assert_eq!(answer.body["status"], "correct");
    let selected = request(&app, Method::POST, "/quizzes", Some(key), json!({})).await;
    assert_eq!(selected.body["account_key"], Value::Null);
    let empty = progress(&app, key).await;
    assert_eq!(empty["history"], json!([]));
    assert_eq!(empty["overall"]["completed_answers"], 0);
    assert_eq!(empty["overall"]["coverage_percentage"], 0.0);
    assert_eq!(empty["overall"]["correct_percentage"], Value::Null);
    assert_eq!(empty["by_topic"]["transcodes"]["completed_answers"], 0);
    let no_key = request(&app, Method::GET, "/progress", None, Value::Null).await;
    assert_eq!(no_key.status, StatusCode::UNAUTHORIZED);
    let no_key_commit = request(
        &app,
        Method::POST,
        "/progress/quizzes",
        None,
        quiz("no-key", &[("q0", true)]),
    )
    .await;
    assert_eq!(no_key_commit.status, StatusCode::UNAUTHORIZED);
    let no_key_reset = request(&app, Method::DELETE, "/progress", None, Value::Null).await;
    assert_eq!(no_key_reset.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn account_key_restores_history_after_a_database_restart() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("progress.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let db = support::database(&url).await;
    let app = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(db.clone()),
    );
    let key = create_account(&app).await;
    commit(&app, &key, quiz("first", &[("q0", false), ("q3", true)])).await;
    let saved = progress(&app, &key).await;
    drop(app);
    db.close().await.unwrap();
    let reopened = support::database(&url).await;
    let app = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(reopened),
    );
    assert_eq!(progress(&app, &key).await, saved);
}

#[tokio::test]
async fn account_keys_scope_history_and_reset_without_a_login_step() {
    let app = support::app(PracticeService::new(support::bank(4))).await;
    let key_a = create_account(&app).await;
    let key_b = create_account(&app).await;
    assert_ne!(key_a, key_b);
    commit(&app, &key_a, quiz("shared-id", &[("q0", true)])).await;
    commit(&app, &key_b, quiz("shared-id", &[("q3", false)])).await;
    let saved_a = progress(&app, &key_a).await;
    let saved_b = progress(&app, &key_b).await;
    assert_ne!(saved_a["history"], saved_b["history"]);
    for key in ["invalid".to_owned(), "0".repeat(64)] {
        let invalid = request(&app, Method::GET, "/progress", Some(&key), Value::Null).await;
        assert_eq!(invalid.status, StatusCode::UNAUTHORIZED);
        assert_eq!(invalid.body["code"], "invalid_account_key");
    }
    let reset = request(&app, Method::DELETE, "/progress", Some(&key_b), Value::Null).await;
    assert_eq!(reset.status, StatusCode::NO_CONTENT);
    assert_eq!(progress(&app, &key_b).await["history"], json!([]));
    assert_eq!(progress(&app, &key_a).await, saved_a);
    assert_eq!(progress(&app, &key_b).await["history"], json!([]));
}

#[tokio::test]
async fn only_complete_distinct_binary_batches_can_commit_and_retries_are_idempotent() {
    let app = support::app(PracticeService::new(support::bank(4))).await;
    let account_key = create_account(&app).await;
    commit(&app, &account_key, quiz("existing", &[("q0", true)])).await;
    let saved = progress(&app, &account_key).await;
    let invalid = [
        quiz("empty", &[]),
        quiz("", &[("q0", true)]),
        quiz("duplicate", &[("q0", true), ("q0", false)]),
        json!({"quiz_id":"partial", "question_ids":["q0","q1"], "outcomes":[{"question_id":"q0","correct":true}]}),
        json!({"quiz_id":"mismatch", "question_ids":["q0"], "outcomes":[{"question_id":"q1","correct":true}]}),
        json!({"quiz_id":"extra", "question_ids":["q0"], "outcomes":[{"question_id":"q0","correct":true},{"question_id":"q1","correct":false}]}),
        json!({"quiz_id":"duplicate-outcome", "question_ids":["q0","q1"], "outcomes":[{"question_id":"q0","correct":true},{"question_id":"q0","correct":false}]}),
        json!({"quiz_id":"not-binary", "question_ids":["q0"], "outcomes":[{"question_id":"q0","correct":null}]}),
        json!({"quiz_id":"answer-archive", "question_ids":["q0"], "outcomes":[{"question_id":"q0","correct":true,"answer":"private"}]}),
        json!({"quiz_id":"grader", "question_ids":["q0"], "outcomes":[{"question_id":"q0","correct":true}], "graded_by":"user"}),
        quiz("unknown", &[("removed", true)]),
    ];
    for body in invalid {
        let rejected = request(
            &app,
            Method::POST,
            "/progress/quizzes",
            Some(&account_key),
            body,
        )
        .await;
        assert!(rejected.status.is_client_error(), "{}", rejected.body);
        assert_eq!(progress(&app, &account_key).await, saved);
    }
    commit(&app, &account_key, quiz("existing", &[("q0", true)])).await;
    assert_eq!(progress(&app, &account_key).await, saved);
    let conflict = request(
        &app,
        Method::POST,
        "/progress/quizzes",
        Some(&account_key),
        quiz("existing", &[("q0", false)]),
    )
    .await;
    assert_eq!(conflict.status, StatusCode::CONFLICT);
    assert_eq!(progress(&app, &account_key).await, saved);
    let batch = quiz("concurrent", &[("q0", false), ("q3", true)]);
    let (a, b) = tokio::join!(
        request(
            &app,
            Method::POST,
            "/progress/quizzes",
            Some(&account_key),
            batch.clone()
        ),
        request(
            &app,
            Method::POST,
            "/progress/quizzes",
            Some(&account_key),
            batch
        )
    );
    assert_eq!(a.status, StatusCode::NO_CONTENT);
    assert_eq!(b.status, StatusCode::NO_CONTENT);
    commit(&app, &account_key, json!({"quiz_id":"concurrent", "question_ids":["q3","q0"], "outcomes":[{"question_id":"q3","correct":true},{"question_id":"q0","correct":false}]})).await;
    assert_eq!(
        progress(&app, &account_key).await["overall"]["completed_answers"],
        3
    );
}

#[tokio::test]
async fn corrected_and_self_assessed_grades_count_once_and_metrics_follow_current_metadata() {
    let db = support::database("sqlite::memory:").await;
    let app = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(db.clone()),
    );
    let account_key = create_account(&app).await;
    let rejected = request(
        &app,
        Method::POST,
        "/answers",
        Some(&account_key),
        json!({"question_id":"q0", "answer":"wrong"}),
    )
    .await;
    assert_eq!(rejected.body["status"], "incorrect");
    let explanation = request(
        &app,
        Method::POST,
        "/answers",
        Some(&account_key),
        json!({"question_id":"q1", "answer":"explanation"}),
    )
    .await;
    assert_eq!(explanation.body["status"], "needs_self_assessment");
    commit(
        &app,
        &account_key,
        quiz(
            "z-completed-first",
            &[("q0", true), ("q1", true), ("q3", false)],
        ),
    )
    .await;
    commit(
        &app,
        &account_key,
        quiz("a-completed-second", &[("q0", false)]),
    )
    .await;
    let saved = progress(&app, &account_key).await;
    assert_eq!(saved["history"][0]["quiz_id"], "z-completed-first");
    assert_eq!(saved["history"][1]["quiz_id"], "a-completed-second");
    assert_eq!(
        saved["overall"],
        json!({"available_questions":4,"covered_questions":3,"coverage_percentage":75.0,"completed_answers":4,"correct_answers":2,"correct_percentage":50.0})
    );
    assert_eq!(saved["by_topic"]["audio"]["covered_questions"], 2);
    assert_eq!(saved["by_topic"]["audio"]["completed_answers"], 3);
    assert!(
        (saved["by_topic"]["audio"]["correct_percentage"]
            .as_f64()
            .unwrap()
            - 200.0 / 3.0)
            .abs()
            < 1e-10
    );
    assert_eq!(saved["by_topic"]["transcodes"]["correct_percentage"], 0.0);
    assert_eq!(saved["recent_outcomes"]["q0"], json!([true, false]));
    let mut changed = support::bank(2);
    changed.questions[0].topic_id = "transcodes".into();
    let updated = routes::api(
        PracticeService::new(changed),
        AccountService::new(db.clone()),
    );
    let metrics = progress(&updated, &account_key).await;
    assert_eq!(metrics["history"], saved["history"]);
    assert_eq!(metrics["overall"]["completed_answers"], 4);
    assert_eq!(metrics["overall"]["coverage_percentage"], 100.0);
    assert_eq!(metrics["by_topic"]["audio"]["completed_answers"], 1);
    assert_eq!(metrics["by_topic"]["transcodes"]["completed_answers"], 2);
    commit(
        &updated,
        &account_key,
        quiz(
            "z-completed-first",
            &[("q0", true), ("q1", true), ("q3", false)],
        ),
    )
    .await;
    let empty = routes::api(
        PracticeService::new(Default::default()),
        AccountService::new(db),
    );
    let metrics = progress(&empty, &account_key).await;
    assert_eq!(metrics["history"], saved["history"]);
    assert_eq!(metrics["overall"]["coverage_percentage"], 0.0);
    assert_eq!(metrics["overall"]["correct_percentage"], 50.0);
    assert_eq!(metrics["by_topic"], json!({}));
}

#[tokio::test]
async fn last_five_saved_outcomes_drive_selection_and_client_history_is_rejected() {
    let app = support::app(PracticeService::new(support::bank(3))).await;
    let account_key = create_account(&app).await;
    for (index, correct) in [false, false, true, true, true, true, true]
        .into_iter()
        .enumerate()
    {
        commit(
            &app,
            &account_key,
            quiz(&format!("quiz-{index}"), &[("q0", correct), ("q1", false)]),
        )
        .await;
    }
    let saved = progress(&app, &account_key).await;
    assert_eq!(saved["recent_outcomes"]["q0"], json!(vec![true; 5]));
    assert_eq!(saved["recent_outcomes"]["q1"], json!(vec![false; 5]));
    assert_eq!(saved["history"].as_array().unwrap().len(), 7);
    let spoof = request(
        &app,
        Method::POST,
        "/quizzes",
        Some(&account_key),
        json!({"recent_outcomes":{"q0":[false]}}),
    )
    .await;
    assert_eq!(spoof.status, StatusCode::UNPROCESSABLE_ENTITY);
    let mut counts = [0; 3];
    for _ in 0..300 {
        let selected = request(
            &app,
            Method::POST,
            "/quizzes",
            Some(&account_key),
            json!({}),
        )
        .await;
        assert_eq!(selected.status, StatusCode::OK);
        let id = selected.body["questions"][0]["question_id"]
            .as_str()
            .unwrap();
        counts[id[1..].parse::<usize>().unwrap()] += 1;
    }
    assert!(counts[0] > 0);
    assert!(counts[1] > counts[0] * 2, "{counts:?}");
    assert!(counts[2] > counts[0] * 2, "{counts:?}");
}

#[tokio::test]
async fn failed_writes_roll_back_the_whole_batch_and_failed_loads_report_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("progress.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let db = support::database(&url).await;
    let app = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(db.clone()),
    );
    let account_key = create_account(&app).await;
    commit(&app, &account_key, quiz("retained", &[("q0", true)])).await;
    let saved = progress(&app, &account_key).await;
    // Limit a quiz to one outcome so the second insert fails after the first succeeds.
    let schema = SchemaManager::new(&db);
    schema
        .create_index(
            Index::create()
                .name("fail_batch_after_first_outcome")
                .table(question_outcome::Entity)
                .col(question_outcome::Column::CompletedQuizId)
                .unique()
                .to_owned(),
        )
        .await
        .unwrap();
    let batch = quiz("retry-after-failure", &[("q0", false), ("q3", true)]);
    let failed = request(
        &app,
        Method::POST,
        "/progress/quizzes",
        Some(&account_key),
        batch.clone(),
    )
    .await;
    assert_eq!(failed.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(failed.body["code"], "database_unavailable");
    assert_eq!(progress(&app, &account_key).await, saved);
    schema
        .drop_index(
            Index::drop()
                .name("fail_batch_after_first_outcome")
                .table(question_outcome::Entity)
                .to_owned(),
        )
        .await
        .unwrap();
    commit(&app, &account_key, batch).await;
    let saved = progress(&app, &account_key).await;
    assert_eq!(saved["overall"]["completed_answers"], 3);
    let read_only = support::connect(&format!("sqlite://{}?mode=ro", path.display())).await;
    let read_only_app = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(read_only.clone()),
    );
    let reset = request(
        &read_only_app,
        Method::DELETE,
        "/progress",
        Some(&account_key),
        Value::Null,
    )
    .await;
    assert_eq!(reset.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(progress(&read_only_app, &account_key).await, saved);
    assert_eq!(progress(&app, &account_key).await, saved);
    read_only.close().await.unwrap();
    db.close().await.unwrap();
    let failed = request(
        &app,
        Method::GET,
        "/progress",
        Some(&account_key),
        Value::Null,
    )
    .await;
    assert_eq!(failed.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(failed.body["code"], "database_unavailable");
    let recovered = routes::api(
        PracticeService::new(support::bank(4)),
        AccountService::new(support::database(&url).await),
    );
    assert_eq!(progress(&recovered, &account_key).await, saved);
}

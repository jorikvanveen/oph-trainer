mod support;

use std::collections::{BTreeMap, HashSet};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode, header},
};
use oph_trainer_backend::{
    accounts::AccountService,
    dto::{CommitQuizDto, QuestionOutcomeDto, QuizSelectionRequestDto},
    practice::PracticeService,
    question_bank::QuestionBank,
};
use rand::{SeedableRng, rngs::StdRng};
use serde_json::{Value, json};
use support::{app, bank};
use tower::ServiceExt;

fn service(size: usize) -> PracticeService {
    PracticeService::new(bank(size))
}

async fn request(app: Router, method: Method, uri: &str, body: &str) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_owned()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let is_json = response.headers()[header::CONTENT_TYPE] == "application/json";
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body = if is_json {
        serde_json::from_slice(&body).unwrap()
    } else {
        Value::String(String::from_utf8(body.to_vec()).unwrap())
    };
    (status, body)
}

#[tokio::test]
async fn catalog_topics_drive_selection_with_distinct_questions_and_small_pools() {
    let app = app(service(12)).await;
    let (status, catalog) = request(app.clone(), Method::GET, "/catalog", "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(catalog["questions"].as_array().unwrap().len(), 12);
    let topic_id = catalog["topics"][0]["topic_id"].as_str().unwrap();
    let (status, selected) = request(
        app.clone(),
        Method::POST,
        "/quizzes",
        &json!({ "topic_ids": [topic_id] }).to_string(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let selected = selected["questions"].as_array().unwrap();
    assert_eq!(selected.len(), 3);
    assert!(
        selected
            .iter()
            .all(|question| question["topic_id"] == topic_id)
    );

    let (status, general) = request(app.clone(), Method::POST, "/quizzes", "{}").await;
    assert_eq!(status, StatusCode::OK);
    let questions = general["questions"].as_array().unwrap();
    assert_eq!(questions.len(), 10);
    assert_eq!(
        questions
            .iter()
            .map(|question| &question["question_id"])
            .collect::<HashSet<_>>()
            .len(),
        10
    );
    assert!(
        questions
            .iter()
            .any(|question| question["topic_id"] == "audio")
    );
    assert!(
        questions
            .iter()
            .any(|question| question["topic_id"] == "transcodes")
    );

    let (status, another) = request(
        app.clone(),
        Method::POST,
        "/quizzes",
        r#"{"topic_ids":null}"#,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(another["questions"].as_array().unwrap().len(), 10);
    assert_ne!(general["quiz_id"], another["quiz_id"]);
}

#[tokio::test]
async fn weighting_favors_unanswered_and_saved_misses_without_excluding_correct_answers() {
    let practice = service(3);
    let request = QuizSelectionRequestDto { topic_ids: None };
    let accounts = AccountService::new(support::database("sqlite::memory:").await);
    let key = accounts.create().await.unwrap().account_key;
    let other_key = accounts.create().await.unwrap().account_key;
    for index in 0..7 {
        for (key, correct) in [(&key, index >= 2), (&other_key, false)] {
            accounts
                .commit_quiz(
                    key,
                    CommitQuizDto {
                        quiz_id: format!("quiz-{index}"),
                        question_ids: vec!["q0".into(), "q1".into()],
                        outcomes: vec![
                            QuestionOutcomeDto {
                                question_id: "q0".into(),
                                correct,
                            },
                            QuestionOutcomeDto {
                                question_id: "q1".into(),
                                correct: false,
                            },
                        ],
                    },
                    &practice.catalog(),
                )
                .await
                .unwrap();
        }
    }
    let history = accounts.recent_outcomes(&key).await.unwrap();
    let other_history = accounts.recent_outcomes(&other_key).await.unwrap();
    let mut counts = [0; 3];
    let mut other_count = 0;
    let mut rng = StdRng::seed_from_u64(42);
    let mut other_rng = StdRng::seed_from_u64(42);
    for _ in 0..1000 {
        let selection = practice
            .select_quiz(request.clone(), &history, &mut rng)
            .unwrap();
        let id = &selection.questions[0].question_id;
        counts[id[1..].parse::<usize>().unwrap()] += 1;
        let other = practice
            .select_quiz(request.clone(), &other_history, &mut other_rng)
            .unwrap();
        other_count += usize::from(other.questions[0].question_id == "q0");
    }
    assert!(counts[0] > 0);
    assert!(counts[1] > counts[0] * 2);
    assert!(counts[2] > counts[0] * 2);
    assert!(other_count > counts[0] * 2);
}

#[tokio::test]
async fn repeated_and_multiple_topics_select_distinct_questions_from_the_combined_pool() {
    let app = app(service(12)).await;
    for (topics, expected_size) in [
        (json!(["audio", "audio"]), 3),
        (json!(["audio", "transcodes"]), 10),
    ] {
        let (status, quiz) = request(
            app.clone(),
            Method::POST,
            "/quizzes",
            &json!({"topic_ids": topics}).to_string(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let questions = quiz["questions"].as_array().unwrap();
        assert_eq!(questions.len(), expected_size);
        assert_eq!(
            questions
                .iter()
                .map(|q| &q["question_id"])
                .collect::<HashSet<_>>()
                .len(),
            expected_size
        );
        if expected_size == 3 {
            assert!(questions.iter().all(|q| q["topic_id"] == "audio"));
        }
    }
}

#[test]
fn selection_ignores_older_than_five_outcomes_and_unknown_historical_questions() {
    let practice = service(12);
    let request = QuizSelectionRequestDto { topic_ids: None };
    let recent = BTreeMap::from([
        ("q0".into(), vec![true, false, true, false, true]),
        ("q1".into(), vec![false; 5]),
    ]);
    let mut extended = recent.clone();
    extended.insert(
        "q0".into(),
        vec![false, false, true, false, true, false, true],
    );
    extended.insert("removed-question".into(), vec![false; 20]);
    for seed in 0..20 {
        let a = practice
            .select_quiz(request.clone(), &recent, &mut StdRng::seed_from_u64(seed))
            .unwrap();
        let b = practice
            .select_quiz(request.clone(), &extended, &mut StdRng::seed_from_u64(seed))
            .unwrap();
        assert_eq!(a, b);
    }
}

#[tokio::test]
async fn short_answers_use_authored_variants_and_explanations_require_self_assessment() {
    let app = app(service(3)).await;
    for (answer, expected) in [
        ("accepted answer", "correct"),
        (" \tAcCePtEd AnSwEr\n", "correct"),
        (" alias ", "correct"),
        ("accepted  answer", "incorrect"),
        ("accepted answr", "incorrect"),
    ] {
        let (status, feedback) = request(
            app.clone(),
            Method::POST,
            "/answers",
            &json!({ "question_id": "q0", "answer": answer }).to_string(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(feedback["status"], expected);
        assert_eq!(feedback["reference_answer"], "Authored reference");
        assert_eq!(feedback["source_url"], "https://example.com/syllabus");
    }
    let (status, feedback) = request(
        app,
        Method::POST,
        "/answers",
        &json!({ "question_id": "q1", "answer": "An explanation" }).to_string(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(feedback["status"], "needs_self_assessment");
    assert_eq!(feedback["reference_answer"], "Authored reference");
    assert_eq!(feedback["source_url"], "https://example.com/syllabus");
}

#[tokio::test]
async fn invalid_requests_return_actionable_errors_instead_of_falling_back_to_general_practice() {
    let app = app(service(3)).await;
    for (uri, body, expected_status, expected_code) in [
        (
            "/quizzes",
            r#"{"topic_ids":[]}"#,
            StatusCode::BAD_REQUEST,
            Some("invalid_topics"),
        ),
        (
            "/quizzes",
            r#"{"topic_ids":["unknown"]}"#,
            StatusCode::BAD_REQUEST,
            Some("invalid_topics"),
        ),
        (
            "/quizzes",
            r#"{"recent_outcomes":{"q0":[]}}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        ),
        ("/quizzes", "{", StatusCode::BAD_REQUEST, None),
        (
            "/quizzes",
            r#"{"topic_ids":"audio"}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        ),
        (
            "/answers",
            r#"{"question_id":"q0","answer":null}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        ),
        (
            "/answers",
            r#"{"question_id":"q0"}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        ),
        ("/answers", "{", StatusCode::BAD_REQUEST, None),
        (
            "/answers",
            r#"{"question_id":"q0","answer":"wrong","grading":{"type":"short_answer","accepted_answers":["wrong"],"aliases":[]}}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        ),
        (
            "/answers",
            r#"{"question_id":"unknown","answer":"answer"}"#,
            StatusCode::NOT_FOUND,
            Some("unknown_question"),
        ),
        (
            "/answers",
            r#"{"question_id":"q0","answer":"  "}"#,
            StatusCode::BAD_REQUEST,
            Some("invalid_answer"),
        ),
    ] {
        let (status, error) = request(app.clone(), Method::POST, uri, body).await;
        assert_eq!(status, expected_status);
        if let Some(expected_code) = expected_code {
            assert_eq!(error["code"], expected_code);
            assert!(!error["message"].as_str().unwrap().is_empty());
        } else {
            assert!(!error.as_str().unwrap().is_empty());
        }
    }
}

#[tokio::test]
async fn an_unpopulated_bank_returns_an_explicit_unavailable_error() {
    let practice = PracticeService::new(QuestionBank::default());
    let (status, error) = request(app(practice).await, Method::POST, "/quizzes", "{}").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(error["code"], "no_questions_available");
}

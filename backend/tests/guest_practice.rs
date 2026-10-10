use std::collections::{BTreeMap, HashSet};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use oph_trainer_backend::{
    dto::{QuestionDto, QuizSelectionRequestDto, TopicDto},
    practice::PracticeService,
    question_bank::{AnswerCriteria, QuestionBank},
    routes,
};
use rand::{SeedableRng, rngs::StdRng};
use serde_json::{Value, json};
use tower::ServiceExt;

fn bank(size: usize) -> QuestionBank {
    QuestionBank {
        topics: vec![
            TopicDto {
                topic_id: "audio".into(),
                name: "Audio".into(),
            },
            TopicDto {
                topic_id: "transcodes".into(),
                name: "Transcodes".into(),
            },
        ],
        questions: (0..size)
            .map(|index| QuestionDto {
                question_id: format!("q{index}"),
                topic_id: if index < 3 { "audio" } else { "transcodes" }.into(),
                prompt: format!("Question {index}"),
                reference_answer: "Authored reference".into(),
                source_url: "https://example.com/syllabus".into(),
                grading: if index == 1 {
                    AnswerCriteria::Explanation {
                        required_points: vec!["Explain why".into()],
                        optional_points: vec![],
                        contradictions: vec![],
                    }
                } else {
                    AnswerCriteria::ShortAnswer {
                        accepted_answers: vec![" Accepted Answer ".into()],
                        aliases: vec![" ALIAS ".into()],
                    }
                },
            })
            .collect(),
    }
}

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
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}

#[tokio::test]
async fn catalog_topics_drive_selection_with_distinct_questions_and_small_pools() {
    let app = routes::api(service(12));
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

#[test]
fn weighting_favors_unanswered_and_missed_questions_without_excluding_correct_answers() {
    let practice = service(3);
    let request = QuizSelectionRequestDto {
        topic_ids: None,
        recent_outcomes: BTreeMap::from([
            ("q0".into(), vec![true; 5]),
            ("q1".into(), vec![false; 5]),
        ]),
    };
    let mut counts = [0; 3];
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..1000 {
        let selection = practice.select_quiz(request.clone(), &mut rng).unwrap();
        let id = &selection.questions[0].question_id;
        counts[id[1..].parse::<usize>().unwrap()] += 1;
    }
    assert!(counts[0] > 0);
    assert!(counts[1] > counts[0] * 2);
    assert!(counts[2] > counts[0] * 2);
}

#[test]
fn selection_ignores_older_than_five_outcomes_and_unknown_historical_questions() {
    let practice = service(12);
    let recent = QuizSelectionRequestDto {
        topic_ids: None,
        recent_outcomes: BTreeMap::from([
            ("q0".into(), vec![true, false, true, false, true]),
            ("q1".into(), vec![false; 5]),
        ]),
    };
    let mut extended = recent.clone();
    extended.recent_outcomes.insert(
        "q0".into(),
        vec![false, false, true, false, true, false, true],
    );
    extended
        .recent_outcomes
        .insert("removed-question".into(), vec![false; 20]);
    for seed in 0..20 {
        let a = practice
            .select_quiz(recent.clone(), &mut StdRng::seed_from_u64(seed))
            .unwrap();
        let b = practice
            .select_quiz(extended.clone(), &mut StdRng::seed_from_u64(seed))
            .unwrap();
        assert_eq!(a, b);
    }
}

#[tokio::test]
async fn short_answers_use_authored_variants_and_explanations_require_self_assessment() {
    let app = routes::api(service(3));
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
    let app = routes::api(service(3));
    for (uri, body, expected_status, expected_code) in [
        (
            "/quizzes",
            r#"{"topic_ids":[]}"#,
            StatusCode::BAD_REQUEST,
            "invalid_topics",
        ),
        (
            "/quizzes",
            r#"{"topic_ids":["unknown"]}"#,
            StatusCode::BAD_REQUEST,
            "invalid_topics",
        ),
        (
            "/quizzes",
            r#"{"recent_outcomes":{"q0":[]}}"#,
            StatusCode::BAD_REQUEST,
            "invalid_history",
        ),
        (
            "/answers",
            r#"{"question_id":"unknown","answer":"answer"}"#,
            StatusCode::NOT_FOUND,
            "unknown_question",
        ),
        (
            "/answers",
            r#"{"question_id":"q0","answer":"  "}"#,
            StatusCode::BAD_REQUEST,
            "invalid_answer",
        ),
    ] {
        let (status, error) = request(app.clone(), Method::POST, uri, body).await;
        assert_eq!(status, expected_status);
        assert_eq!(error["code"], expected_code);
        assert!(!error["message"].as_str().unwrap().is_empty());
    }
}

#[tokio::test]
async fn an_unpopulated_bank_returns_an_explicit_unavailable_error() {
    let practice = PracticeService::new(QuestionBank::default());
    let (status, error) = request(routes::api(practice), Method::POST, "/quizzes", "{}").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(error["code"], "no_questions_available");
}

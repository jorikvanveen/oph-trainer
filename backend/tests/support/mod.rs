use axum::Router;
use migration::{Migrator, MigratorTrait};
use oph_trainer_backend::{
    accounts::AccountService,
    dto::{QuestionDto, TopicDto},
    practice::PracticeService,
    question_bank::{AnswerCriteria, QuestionBank},
    routes,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn database(url: &str) -> DatabaseConnection {
    let db = connect(url).await;
    Migrator::up(&db, None).await.unwrap();
    db
}

pub async fn connect(url: &str) -> DatabaseConnection {
    let mut options = ConnectOptions::new(url);
    options.sqlx_logging(false).record_stmt_in_spans(false);
    Database::connect(options).await.unwrap()
}

pub async fn app(practice: PracticeService) -> Router {
    routes::api(
        practice,
        AccountService::new(database("sqlite::memory:").await),
    )
}

pub fn bank(size: usize) -> QuestionBank {
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

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use crate::{
    dto::{
        AnswerFeedbackDto, AnswerSubmissionDto, ApiErrorCodeDto, ApiErrorDto, QuestionCatalogDto,
        QuizSelectionRequestDto, QuizSelectionResponseDto, SampleDto,
    },
    practice::PracticeService,
};

pub fn api(practice: PracticeService) -> Router {
    Router::new()
        .route("/sample", get(sample))
        .route("/catalog", get(catalog))
        .route("/quizzes", post(select_quiz))
        .route("/answers", post(answer))
        .fallback(|| async { StatusCode::NOT_FOUND })
        .with_state(Arc::new(practice))
}

pub async fn sample() -> Json<SampleDto> {
    Json(SampleDto {
        message: "Hello from Orpheus interview trainer".to_owned(),
    })
}

async fn catalog(State(practice): State<Arc<PracticeService>>) -> Json<QuestionCatalogDto> {
    Json(practice.catalog())
}

async fn select_quiz(
    State(practice): State<Arc<PracticeService>>,
    Json(request): Json<QuizSelectionRequestDto>,
) -> Result<Json<QuizSelectionResponseDto>, PracticeApiError> {
    Ok(Json(practice.select_quiz(request, &mut rand::rng())?))
}

async fn answer(
    State(practice): State<Arc<PracticeService>>,
    Json(submission): Json<AnswerSubmissionDto>,
) -> Result<Json<AnswerFeedbackDto>, PracticeApiError> {
    Ok(Json(practice.grade_answer(submission)?))
}

struct PracticeApiError(ApiErrorDto);

impl From<ApiErrorDto> for PracticeApiError {
    fn from(error: ApiErrorDto) -> Self {
        Self(error)
    }
}

impl IntoResponse for PracticeApiError {
    fn into_response(self) -> Response {
        let status = match self.0.code {
            ApiErrorCodeDto::UnknownQuestion => StatusCode::NOT_FOUND,
            ApiErrorCodeDto::NoQuestionsAvailable => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
        };
        (status, Json(self.0)).into_response()
    }
}

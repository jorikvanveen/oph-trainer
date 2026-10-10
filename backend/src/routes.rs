use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};

use crate::{
    accounts::AccountService,
    dto::{
        AccountDto, AnswerFeedbackDto, AnswerSubmissionDto, ApiErrorCodeDto, ApiErrorDto,
        CommitQuizDto, ProgressDto, QuestionCatalogDto, QuizSelectionRequestDto, SampleDto,
    },
    practice::PracticeService,
};

struct ApiState {
    practice: PracticeService,
    accounts: AccountService,
}

pub fn api(practice: PracticeService, accounts: AccountService) -> Router {
    Router::new()
        .route("/sample", get(sample))
        .route("/catalog", get(catalog))
        .route("/quizzes", post(select_quiz))
        .route("/answers", post(answer))
        .route("/accounts", post(create_account))
        .route("/progress", get(progress).delete(reset_progress))
        .route("/progress/quizzes", post(commit_quiz))
        .fallback(|| async { StatusCode::NOT_FOUND })
        .layer(axum::middleware::map_response(disable_caching))
        .with_state(Arc::new(ApiState { practice, accounts }))
}

async fn disable_caching(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response
}

pub async fn sample() -> Json<SampleDto> {
    Json(SampleDto {
        message: "Hello from Orpheus interview trainer".to_owned(),
    })
}

async fn catalog(State(state): State<Arc<ApiState>>) -> Json<QuestionCatalogDto> {
    Json(state.practice.catalog())
}

async fn create_account(State(state): State<Arc<ApiState>>) -> Result<Json<AccountDto>, ApiError> {
    Ok(Json(state.accounts.create().await?))
}

async fn select_quiz(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
    Json(request): Json<QuizSelectionRequestDto>,
) -> Result<Response, ApiError> {
    let account_key = state.accounts.current(&headers).await?;
    let recent = match &account_key {
        Some(account_key) => state.accounts.recent_outcomes(account_key).await?,
        None => Default::default(),
    };
    let mut quiz = state
        .practice
        .select_quiz(request, &recent, &mut rand::rng())?;
    quiz.account_key = match account_key {
        Some(_) => None,
        None => Some(state.accounts.create().await?.account_key),
    };
    Ok(Json(quiz).into_response())
}

async fn answer(
    State(state): State<Arc<ApiState>>,
    Json(submission): Json<AnswerSubmissionDto>,
) -> Result<Json<AnswerFeedbackDto>, ApiError> {
    Ok(Json(state.practice.grade_answer(submission)?))
}

async fn progress(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
) -> Result<Json<ProgressDto>, ApiError> {
    let account_key = state.accounts.require(&headers).await?;
    Ok(Json(
        state
            .accounts
            .progress(&account_key, &state.practice.catalog())
            .await?,
    ))
}

async fn commit_quiz(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
    Json(request): Json<CommitQuizDto>,
) -> Result<StatusCode, ApiError> {
    let account_key = state.accounts.require(&headers).await?;
    state
        .accounts
        .commit_quiz(&account_key, request, &state.practice.catalog())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn reset_progress(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let account_key = state.accounts.require(&headers).await?;
    state.accounts.reset_progress(&account_key).await?;
    Ok(StatusCode::NO_CONTENT)
}

struct ApiError(ApiErrorDto);

impl From<ApiErrorDto> for ApiError {
    fn from(error: ApiErrorDto) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0.code {
            ApiErrorCodeDto::UnknownQuestion => StatusCode::NOT_FOUND,
            ApiErrorCodeDto::NoQuestionsAvailable
            | ApiErrorCodeDto::DatabaseUnavailable
            | ApiErrorCodeDto::AccountUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ApiErrorCodeDto::InvalidAccountKey | ApiErrorCodeDto::Unauthenticated => {
                StatusCode::UNAUTHORIZED
            }
            ApiErrorCodeDto::QuizConflict => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        };
        (status, Json(self.0)).into_response()
    }
}

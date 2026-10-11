use std::{net::SocketAddr, sync::Arc};

use axum::{
    Extension, Json, Router,
    extract::{ConnectInfo, DefaultBodyLimit, State},
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
    grading::{ANSWER_CHARS, GradingService, REQUEST_BYTES},
    practice::PracticeService,
};

struct ApiState {
    practice: PracticeService,
    accounts: AccountService,
    grading: GradingService,
    use_x_forwarded_for: bool,
}

pub fn api(practice: PracticeService, accounts: AccountService) -> Router {
    api_with_grading(
        practice,
        accounts,
        GradingService::new(None).expect("grading HTTP client"),
        false,
    )
}

pub fn api_with_grading(
    practice: PracticeService,
    accounts: AccountService,
    grading: GradingService,
    use_x_forwarded_for: bool,
) -> Router {
    Router::new()
        .route("/sample", get(sample))
        .route("/catalog", get(catalog))
        .route("/quizzes", post(select_quiz))
        .route(
            "/answers",
            post(answer).layer(DefaultBodyLimit::max(REQUEST_BYTES)),
        )
        .route("/accounts", post(create_account))
        .route("/progress", get(progress).delete(reset_progress))
        .route("/progress/quizzes", post(commit_quiz))
        .fallback(|| async { StatusCode::NOT_FOUND })
        .layer(axum::middleware::map_response(disable_caching))
        .layer(axum::middleware::from_fn(crate::logging::log_request))
        .with_state(Arc::new(ApiState {
            practice,
            accounts,
            grading,
            use_x_forwarded_for,
        }))
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
    let selected_topics = request
        .topic_ids
        .as_ref()
        .map(|ids| ids.iter().collect::<std::collections::HashSet<_>>().len());
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
    tracing::info!(
        questions = quiz.questions.len(),
        selected_topics,
        general_practice = selected_topics.is_none(),
        questions_with_history = recent.len(),
        account_created = quiz.account_key.is_some(),
        "Quiz selected"
    );
    Ok(Json(quiz).into_response())
}

async fn answer(
    State(state): State<Arc<ApiState>>,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Json(submission): Json<AnswerSubmissionDto>,
) -> Result<Json<AnswerFeedbackDto>, ApiError> {
    if submission.answer.chars().count() > ANSWER_CHARS {
        return Err(ApiErrorDto {
            code: ApiErrorCodeDto::InvalidAnswer,
            message: "Answer is too long. Shorten it or use self-assessment.".into(),
        }
        .into());
    }
    let mut feedback = state.practice.grade_answer(submission.clone())?;
    if feedback.status == crate::dto::FeedbackStatusDto::NeedsSelfAssessment {
        let socket_ip = peer.map(|peer| peer.0.0.ip());
        let client_ip = if state.use_x_forwarded_for {
            headers
                .get("x-forwarded-for")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.split(',').next()?.trim().parse().ok())
                .or(socket_ip)
        } else {
            socket_ip
        };
        let question = state
            .practice
            .question(&submission.question_id)
            .expect("validated question");
        if let Some(assessment) = state
            .grading
            .grade(question, &submission.answer, client_ip)
            .await
        {
            feedback.status = if assessment.required_points.iter().all(|point| point.covered)
                && assessment.contradictions.is_empty()
            {
                crate::dto::FeedbackStatusDto::Correct
            } else {
                crate::dto::FeedbackStatusDto::Incorrect
            };
            feedback.explanation_feedback = Some(assessment);
        }
    }
    tracing::info!(
        question_id = %feedback.question_id,
        status = ?feedback.status,
        automatic_explanation_grading = feedback.explanation_feedback.is_some(),
        "Answer assessed"
    );
    Ok(Json(feedback))
}

async fn progress(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
) -> Result<Json<ProgressDto>, ApiError> {
    let account_key = state.accounts.require(&headers).await?;
    let progress = state
        .accounts
        .progress(&account_key, &state.practice.catalog())
        .await?;
    tracing::debug!(
        completed_quizzes = progress.history.len(),
        completed_answers = progress.overall.completed_answers,
        "Progress loaded"
    );
    Ok(Json(progress))
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
        tracing::debug!(code = ?self.0.code, "API operation rejected");
        (status, Json(self.0)).into_response()
    }
}

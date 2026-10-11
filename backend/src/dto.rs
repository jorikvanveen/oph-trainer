use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::question_bank::AnswerCriteria;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SampleDto {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TopicDto {
    pub topic_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuestionMetadataDto {
    pub question_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuestionCatalogDto {
    pub topics: Vec<TopicDto>,
    pub questions: Vec<QuestionMetadataDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuestionDto {
    pub question_id: String,
    pub topic_id: String,
    pub prompt: String,
    pub reference_answer: String,
    pub source_url: String,
    pub grading: AnswerCriteria,
    #[serde(default)]
    pub material: Vec<ExerciseMaterialDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[ts(export)]
pub enum ExerciseMaterialDto {
    Image { src: String, description: String },
    Excerpt { description: String, text: String },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuizSelectionRequestDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional = nullable)]
    pub topic_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuizSelectionResponseDto {
    pub quiz_id: String,
    pub questions: Vec<QuestionDto>,
    pub account_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct AnswerSubmissionDto {
    pub question_id: String,
    pub answer: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum FeedbackStatusDto {
    Correct,
    Incorrect,
    NeedsSelfAssessment,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct AnswerFeedbackDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional = nullable)]
    pub explanation_feedback: Option<ExplanationFeedbackDto>,
    pub question_id: String,
    pub status: FeedbackStatusDto,
    pub reference_answer: String,
    pub source_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ExplanationFeedbackDto {
    pub required_points: Vec<RequiredPointFeedbackDto>,
    pub contradictions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct RequiredPointFeedbackDto {
    pub point: String,
    pub covered: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct QuestionOutcomeDto {
    pub question_id: String,
    pub correct: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct CompletedQuizDto {
    pub quiz_id: String,
    pub outcomes: Vec<QuestionOutcomeDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct CommitQuizDto {
    pub quiz_id: String,
    pub question_ids: Vec<String>,
    pub outcomes: Vec<QuestionOutcomeDto>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct AccountDto {
    pub account_key: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ProgressMetricsDto {
    pub available_questions: usize,
    pub covered_questions: usize,
    pub coverage_percentage: f64,
    pub completed_answers: usize,
    pub correct_answers: usize,
    pub correct_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ProgressDto {
    pub history: Vec<CompletedQuizDto>,
    pub recent_outcomes: BTreeMap<String, Vec<bool>>,
    pub overall: ProgressMetricsDto,
    pub by_topic: BTreeMap<String, ProgressMetricsDto>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ApiErrorCodeDto {
    NoQuestionsAvailable,
    InvalidTopics,
    InvalidHistory,
    UnknownQuestion,
    InvalidAnswer,
    InvalidCompletedQuiz,
    QuizConflict,
    InvalidAccountKey,
    Unauthenticated,
    DatabaseUnavailable,
    AccountUnavailable,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ApiErrorDto {
    pub code: ApiErrorCodeDto,
    pub message: String,
}

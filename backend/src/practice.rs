use std::collections::{BTreeMap, HashSet};

use rand::{Rng, distr::Distribution, distr::weighted::WeightedIndex};

use crate::{
    dto::{
        AnswerFeedbackDto, AnswerSubmissionDto, ApiErrorCodeDto, ApiErrorDto, FeedbackStatusDto,
        QuestionCatalogDto, QuestionMetadataDto, QuizSelectionRequestDto, QuizSelectionResponseDto,
    },
    question_bank::{AnswerCriteria, QuestionBank},
};

const QUIZ_SIZE: usize = 10;

pub struct PracticeService {
    bank: QuestionBank,
}

impl PracticeService {
    pub fn new(bank: QuestionBank) -> Self {
        Self { bank }
    }

    pub fn catalog(&self) -> QuestionCatalogDto {
        QuestionCatalogDto {
            topics: self.bank.topics.clone(),
            questions: self
                .bank
                .questions
                .iter()
                .map(|question| QuestionMetadataDto {
                    question_id: question.question_id.clone(),
                    topic_id: question.topic_id.clone(),
                })
                .collect(),
        }
    }

    pub fn select_quiz(
        &self,
        request: QuizSelectionRequestDto,
        recent_outcomes: &BTreeMap<String, Vec<bool>>,
        rng: &mut impl Rng,
    ) -> Result<QuizSelectionResponseDto, ApiErrorDto> {
        let topics = request
            .topic_ids
            .as_ref()
            .map(|ids| ids.iter().map(String::as_str).collect::<HashSet<_>>());
        if let Some(topics) = &topics {
            if topics.is_empty() {
                return Err(error(
                    ApiErrorCodeDto::InvalidTopics,
                    "Select at least one topic, or omit topic_ids for general practice.",
                ));
            }
            if let Some(unknown) = topics
                .iter()
                .find(|id| !self.bank.topics.iter().any(|topic| topic.topic_id == **id))
            {
                return Err(error(
                    ApiErrorCodeDto::InvalidTopics,
                    format!("Unknown topic ID: {unknown}"),
                ));
            }
        }

        let mut pool = Vec::new();
        for question in &self.bank.questions {
            let outcomes = recent_outcomes
                .get(&question.question_id)
                .map(Vec::as_slice);
            if outcomes.is_some_and(|grades| grades.is_empty()) {
                return Err(error(
                    ApiErrorCodeDto::InvalidHistory,
                    format!(
                        "Recent outcomes for {} must not be empty.",
                        question.question_id
                    ),
                ));
            }
            if topics
                .as_ref()
                .is_none_or(|ids| ids.contains(question.topic_id.as_str()))
            {
                let weight = question_weight(outcomes.unwrap_or_default());
                pool.push((question, weight));
            }
        }
        if pool.is_empty() {
            return Err(error(
                ApiErrorCodeDto::NoQuestionsAvailable,
                "No questions are available for this selection.",
            ));
        }

        let mut questions = Vec::with_capacity(QUIZ_SIZE.min(pool.len()));
        while !pool.is_empty() && questions.len() < QUIZ_SIZE {
            let weights = WeightedIndex::new(pool.iter().map(|(_, weight)| *weight))
                .expect("question weights are between 1 and 5");
            let index = weights.sample(rng);
            let (question, _) = pool.swap_remove(index);
            questions.push(question.clone());
        }
        Ok(QuizSelectionResponseDto {
            quiz_id: format!("{:032x}", rng.random::<u128>()),
            questions,
            account_key: None,
        })
    }

    pub(crate) fn question(&self, id: &str) -> Option<&crate::dto::QuestionDto> {
        self.bank
            .questions
            .iter()
            .find(|question| question.question_id == id)
    }

    pub fn grade_answer(
        &self,
        submission: AnswerSubmissionDto,
    ) -> Result<AnswerFeedbackDto, ApiErrorDto> {
        let question = self
            .bank
            .questions
            .iter()
            .find(|question| question.question_id == submission.question_id)
            .ok_or_else(|| {
                error(
                    ApiErrorCodeDto::UnknownQuestion,
                    format!("Unknown question ID: {}", submission.question_id),
                )
            })?;
        let answer = normalize(&submission.answer);
        if answer.is_empty() {
            return Err(error(
                ApiErrorCodeDto::InvalidAnswer,
                "Enter an answer before submitting.",
            ));
        }
        let status = match &question.grading {
            AnswerCriteria::ShortAnswer {
                accepted_answers,
                aliases,
            } => {
                if accepted_answers
                    .iter()
                    .chain(aliases)
                    .any(|accepted| normalize(accepted) == answer)
                {
                    FeedbackStatusDto::Correct
                } else {
                    FeedbackStatusDto::Incorrect
                }
            }
            AnswerCriteria::Explanation { .. } => FeedbackStatusDto::NeedsSelfAssessment,
        };
        Ok(AnswerFeedbackDto {
            explanation_feedback: None,
            question_id: question.question_id.clone(),
            status,
            reference_answer: question.reference_answer.clone(),
            source_url: question.source_url.clone(),
        })
    }
}

fn question_weight(outcomes: &[bool]) -> f64 {
    let recent = &outcomes[outcomes.len().saturating_sub(5)..];
    let missed_fraction = if recent.is_empty() {
        1.0
    } else {
        recent.iter().filter(|correct| !**correct).count() as f64 / recent.len() as f64
    };
    1.0 + 4.0 * missed_fraction
}

fn normalize(answer: &str) -> String {
    answer.trim().to_lowercase()
}

fn error(code: ApiErrorCodeDto, message: impl Into<String>) -> ApiErrorDto {
    ApiErrorDto {
        code,
        message: message.into(),
    }
}

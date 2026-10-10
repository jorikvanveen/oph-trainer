use std::collections::{BTreeMap, HashSet};

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait, TryInsertResult,
};

use crate::{
    accounts::{AccountService, database_error, error},
    dto::{
        ApiErrorCodeDto, ApiErrorDto, CommitQuizDto, CompletedQuizDto, ProgressDto,
        ProgressMetricsDto, QuestionCatalogDto, QuestionOutcomeDto,
    },
    entity::{completed_quizzes as completed_quiz, question_outcomes as question_outcome},
};

impl AccountService {
    pub async fn history(&self, account_key: &str) -> Result<Vec<CompletedQuizDto>, ApiErrorDto> {
        load_history(self.database(), account_key).await
    }

    pub async fn recent_outcomes(
        &self,
        account_key: &str,
    ) -> Result<BTreeMap<String, Vec<bool>>, ApiErrorDto> {
        Ok(recent_outcomes(&self.history(account_key).await?))
    }

    pub async fn progress(
        &self,
        account_key: &str,
        catalog: &QuestionCatalogDto,
    ) -> Result<ProgressDto, ApiErrorDto> {
        let history = self.history(account_key).await?;
        let all: Vec<_> = history.iter().flat_map(|quiz| &quiz.outcomes).collect();
        let overall = metrics(
            catalog
                .questions
                .iter()
                .map(|q| q.question_id.as_str())
                .collect(),
            &all,
        );
        let by_topic = catalog
            .topics
            .iter()
            .map(|topic| {
                let questions: HashSet<_> = catalog
                    .questions
                    .iter()
                    .filter(|question| question.topic_id == topic.topic_id)
                    .map(|question| question.question_id.as_str())
                    .collect();
                let outcomes: Vec<_> = all
                    .iter()
                    .copied()
                    .filter(|outcome| questions.contains(outcome.question_id.as_str()))
                    .collect();
                (topic.topic_id.clone(), metrics(questions, &outcomes))
            })
            .collect();
        Ok(ProgressDto {
            recent_outcomes: recent_outcomes(&history),
            history,
            overall,
            by_topic,
        })
    }

    pub async fn commit_quiz(
        &self,
        account_key: &str,
        quiz: CommitQuizDto,
        catalog: &QuestionCatalogDto,
    ) -> Result<(), ApiErrorDto> {
        let batch: HashSet<_> = quiz.question_ids.iter().collect();
        let outcomes: HashSet<_> = quiz
            .outcomes
            .iter()
            .map(|outcome| &outcome.question_id)
            .collect();
        if quiz.quiz_id.is_empty()
            || quiz.quiz_id.len() > 128
            || batch.is_empty()
            || batch.len() != quiz.question_ids.len()
            || outcomes.len() != quiz.outcomes.len()
            || batch != outcomes
        {
            return Err(error(
                ApiErrorCodeDto::InvalidCompletedQuiz,
                "A completed quiz needs a quiz ID and exactly one binary outcome for every distinct question in its nonempty batch.",
            ));
        }

        let tx = self.database().begin().await.map_err(database_error)?;
        // Insert first so concurrent retries serialize on the account/quiz uniqueness constraint.
        let inserted = completed_quiz::Entity::insert(completed_quiz::ActiveModel {
            account_key: Set(account_key.to_owned()),
            quiz_id: Set(quiz.quiz_id.clone()),
            ..Default::default()
        })
        .on_conflict_do_nothing_on([
            completed_quiz::Column::AccountKey,
            completed_quiz::Column::QuizId,
        ])
        .exec(&tx)
        .await
        .map_err(database_error)?;
        match inserted {
            TryInsertResult::Conflicted => {
                let previous = completed_quiz::Entity::find()
                    .filter(completed_quiz::Column::AccountKey.eq(account_key))
                    .filter(completed_quiz::Column::QuizId.eq(&quiz.quiz_id))
                    .find_with_related(question_outcome::Entity)
                    .all(&tx)
                    .await
                    .map_err(database_error)?;
                let (_, outcomes) = previous.into_iter().next().ok_or_else(|| {
                    database_error(DbErr::RecordNotFound(
                        "The saved quiz could not be loaded.".into(),
                    ))
                })?;
                let previous: BTreeMap<_, _> = outcomes
                    .iter()
                    .map(|outcome| (&outcome.question_id, outcome.correct))
                    .collect();
                let submitted: BTreeMap<_, _> = quiz
                    .outcomes
                    .iter()
                    .map(|outcome| (&outcome.question_id, outcome.correct))
                    .collect();
                if previous != submitted {
                    return Err(error(
                        ApiErrorCodeDto::QuizConflict,
                        "This quiz was already saved with different outcomes. Saved history is unchanged.",
                    ));
                }
            }
            TryInsertResult::Inserted(inserted) => {
                if batch.iter().any(|id| {
                    !catalog
                        .questions
                        .iter()
                        .any(|question| &question.question_id == *id)
                }) {
                    return Err(error(
                        ApiErrorCodeDto::UnknownQuestion,
                        "The completed batch contains an unknown question ID.",
                    ));
                }
                let completion_id = inserted.last_insert_id;
                let submitted: BTreeMap<_, _> = quiz
                    .outcomes
                    .into_iter()
                    .map(|outcome| (outcome.question_id, outcome.correct))
                    .collect();
                for (position, question_id) in quiz.question_ids.into_iter().enumerate() {
                    question_outcome::Entity::insert(question_outcome::ActiveModel {
                        completed_quiz_id: Set(completion_id),
                        position: Set(position as i64),
                        correct: Set(submitted[&question_id]),
                        question_id: Set(question_id),
                    })
                    .exec(&tx)
                    .await
                    .map_err(database_error)?;
                }
            }
            TryInsertResult::Empty => return Err(database_error(DbErr::RecordNotInserted)),
        }
        tx.commit().await.map_err(database_error)?;
        Ok(())
    }

    pub async fn reset_progress(&self, account_key: &str) -> Result<(), ApiErrorDto> {
        let tx = self.database().begin().await.map_err(database_error)?;
        completed_quiz::Entity::delete_many()
            .filter(completed_quiz::Column::AccountKey.eq(account_key))
            .exec(&tx)
            .await
            .map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        Ok(())
    }
}

async fn load_history(
    db: &impl ConnectionTrait,
    account_key: &str,
) -> Result<Vec<CompletedQuizDto>, ApiErrorDto> {
    let quizzes = completed_quiz::Entity::find()
        .filter(completed_quiz::Column::AccountKey.eq(account_key))
        .find_with_related(question_outcome::Entity)
        .order_by_asc(completed_quiz::Column::Id)
        .order_by_asc(question_outcome::Column::Position)
        .all(db)
        .await
        .map_err(database_error)?;
    Ok(quizzes
        .into_iter()
        .map(|(quiz, outcomes)| CompletedQuizDto {
            quiz_id: quiz.quiz_id,
            outcomes: outcomes
                .into_iter()
                .map(|outcome| QuestionOutcomeDto {
                    question_id: outcome.question_id,
                    correct: outcome.correct,
                })
                .collect(),
        })
        .collect())
}

fn recent_outcomes(history: &[CompletedQuizDto]) -> BTreeMap<String, Vec<bool>> {
    let mut recent: BTreeMap<String, Vec<bool>> = BTreeMap::new();
    for outcome in history.iter().flat_map(|quiz| &quiz.outcomes) {
        let grades = recent.entry(outcome.question_id.clone()).or_default();
        grades.push(outcome.correct);
        if grades.len() > 5 {
            grades.remove(0);
        }
    }
    recent
}

fn metrics(questions: HashSet<&str>, outcomes: &[&QuestionOutcomeDto]) -> ProgressMetricsDto {
    let covered = outcomes
        .iter()
        .map(|outcome| outcome.question_id.as_str())
        .filter(|id| questions.contains(id))
        .collect::<HashSet<_>>()
        .len();
    let correct = outcomes.iter().filter(|outcome| outcome.correct).count();
    ProgressMetricsDto {
        available_questions: questions.len(),
        covered_questions: covered,
        coverage_percentage: if questions.is_empty() {
            0.0
        } else {
            covered as f64 * 100.0 / questions.len() as f64
        },
        completed_answers: outcomes.len(),
        correct_answers: correct,
        correct_percentage: (!outcomes.is_empty())
            .then(|| correct as f64 * 100.0 / outcomes.len() as f64),
    }
}

use std::collections::{BTreeMap, HashSet};

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set,
    TransactionTrait, TryInsertResult,
    sea_query::{
        Alias, Expr, ExprTrait, Func, JoinType, Order, OverStatement, Query, SelectStatement,
        WindowStatement,
    },
};

use crate::{
    accounts::{AccountService, database_error, error},
    dto::{
        ApiErrorCodeDto, ApiErrorDto, CommitQuizDto, ProgressDto, ProgressMetricsDto,
        QuestionCatalogDto,
    },
    entity::{completed_quizzes as completed_quiz, question_outcomes as question_outcome},
};

impl AccountService {
    pub async fn recent_outcomes(
        &self,
        account_key: &str,
    ) -> Result<BTreeMap<String, Vec<bool>>, ApiErrorDto> {
        load_recent_outcomes(self.database(), account_key).await
    }

    pub async fn progress(
        &self,
        account_key: &str,
        catalog: &QuestionCatalogDto,
    ) -> Result<ProgressDto, ApiErrorDto> {
        let tx = self.database().begin().await.map_err(database_error)?;
        let completed_quiz_count = completed_quiz::Entity::find()
            .filter(completed_quiz::Column::AccountKey.eq(account_key))
            .count(&tx)
            .await
            .map_err(database_error)? as usize;
        let questions = catalog
            .questions
            .iter()
            .map(|q| q.question_id.as_str())
            .collect();
        let overall = load_metrics(&tx, account_outcomes(account_key), questions).await?;
        let mut by_topic = BTreeMap::new();
        for topic in &catalog.topics {
            let questions: HashSet<_> = catalog
                .questions
                .iter()
                .filter(|question| question.topic_id == topic.topic_id)
                .map(|question| question.question_id.as_str())
                .collect();
            let mut query = account_outcomes(account_key);
            query.and_where(
                Expr::col((
                    question_outcome::Entity,
                    question_outcome::Column::QuestionId,
                ))
                .is_in(questions.iter().copied()),
            );
            by_topic.insert(
                topic.topic_id.clone(),
                load_metrics(&tx, query, questions).await?,
            );
        }
        let recent_outcomes = load_recent_outcomes(&tx, account_key).await?;
        tx.commit().await.map_err(database_error)?;
        Ok(ProgressDto {
            completed_quiz_count,
            recent_outcomes,
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
        let question_count = quiz.question_ids.len();
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
        let already_saved = matches!(inserted, TryInsertResult::Conflicted);
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
                for outcome in quiz.outcomes {
                    question_outcome::Entity::insert(question_outcome::ActiveModel {
                        completed_quiz_id: Set(completion_id),
                        correct: Set(outcome.correct),
                        question_id: Set(outcome.question_id),
                    })
                    .exec(&tx)
                    .await
                    .map_err(database_error)?;
                }
            }
            TryInsertResult::Empty => return Err(database_error(DbErr::RecordNotInserted)),
        }
        tx.commit().await.map_err(database_error)?;
        tracing::info!(
            questions = question_count,
            already_saved,
            "Completed quiz saved"
        );
        Ok(())
    }

    pub async fn reset_progress(&self, account_key: &str) -> Result<(), ApiErrorDto> {
        let tx = self.database().begin().await.map_err(database_error)?;
        let deleted = completed_quiz::Entity::delete_many()
            .filter(completed_quiz::Column::AccountKey.eq(account_key))
            .exec(&tx)
            .await
            .map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        tracing::info!(completed_quizzes = deleted.rows_affected, "Progress reset");
        Ok(())
    }
}

const RECENT_OUTCOME_LIMIT: u64 = 5;

fn account_outcomes(account_key: &str) -> SelectStatement {
    Query::select()
        .from(question_outcome::Entity)
        .join(
            JoinType::InnerJoin,
            completed_quiz::Entity,
            Expr::col((
                question_outcome::Entity,
                question_outcome::Column::CompletedQuizId,
            ))
            .eq(Expr::col((
                completed_quiz::Entity,
                completed_quiz::Column::Id,
            ))),
        )
        .and_where(completed_quiz::Column::AccountKey.eq(account_key))
        .to_owned()
}

async fn load_recent_outcomes(
    db: &impl ConnectionTrait,
    account_key: &str,
) -> Result<BTreeMap<String, Vec<bool>>, ApiErrorDto> {
    let mut ranked = account_outcomes(account_key);
    ranked
        .columns([
            (
                question_outcome::Entity,
                question_outcome::Column::QuestionId,
            ),
            (question_outcome::Entity, question_outcome::Column::Correct),
            (
                question_outcome::Entity,
                question_outcome::Column::CompletedQuizId,
            ),
        ])
        .expr_window_as(
            Func::cust(Alias::new("ROW_NUMBER")),
            WindowStatement::new()
                .partition_by((
                    question_outcome::Entity,
                    question_outcome::Column::QuestionId,
                ))
                .order_by(
                    (
                        question_outcome::Entity,
                        question_outcome::Column::CompletedQuizId,
                    ),
                    Order::Desc,
                )
                .to_owned(),
            Alias::new("outcome_rank"),
        );
    let query = Query::select()
        .columns([
            question_outcome::Column::QuestionId,
            question_outcome::Column::Correct,
        ])
        .from_subquery(ranked, Alias::new("ranked_outcomes"))
        .and_where(Expr::col(Alias::new("outcome_rank")).lte(RECENT_OUTCOME_LIMIT))
        .order_by(question_outcome::Column::QuestionId, Order::Asc)
        .order_by(question_outcome::Column::CompletedQuizId, Order::Asc)
        .to_owned();
    let rows = db.query_all(&query).await.map_err(database_error)?;
    let mut recent: BTreeMap<String, Vec<bool>> = BTreeMap::new();
    for row in rows {
        let id = row.try_get("", "question_id").map_err(database_error)?;
        let correct = row.try_get("", "correct").map_err(database_error)?;
        recent.entry(id).or_default().push(correct);
    }
    Ok(recent)
}

async fn load_metrics(
    db: &impl ConnectionTrait,
    mut query: SelectStatement,
    questions: HashSet<&str>,
) -> Result<ProgressMetricsDto, ApiErrorDto> {
    let question_id = Expr::col((
        question_outcome::Entity,
        question_outcome::Column::QuestionId,
    ));
    query
        .expr_as(question_id.clone().count(), Alias::new("completed_answers"))
        .expr_as(
            Func::count(
                Expr::case(
                    Expr::col((question_outcome::Entity, question_outcome::Column::Correct))
                        .eq(true),
                    1,
                )
                .finally(Option::<i32>::None),
            ),
            Alias::new("correct_answers"),
        )
        .expr_as(
            Func::count_distinct(
                Expr::case(
                    question_id.clone().is_in(questions.iter().copied()),
                    question_id,
                )
                .finally(Option::<String>::None),
            ),
            Alias::new("covered_questions"),
        );
    let row = db
        .query_one(&query)
        .await
        .map_err(database_error)?
        .ok_or_else(|| {
            database_error(DbErr::RecordNotFound(
                "Progress aggregates were not returned.".into(),
            ))
        })?;
    let completed = row
        .try_get::<i64>("", "completed_answers")
        .map_err(database_error)? as usize;
    let correct = row
        .try_get::<i64>("", "correct_answers")
        .map_err(database_error)? as usize;
    let covered = row
        .try_get::<i64>("", "covered_questions")
        .map_err(database_error)? as usize;
    Ok(ProgressMetricsDto {
        available_questions: questions.len(),
        covered_questions: covered,
        coverage_percentage: if questions.is_empty() {
            0.0
        } else {
            covered as f64 * 100.0 / questions.len() as f64
        },
        completed_answers: completed,
        correct_answers: correct,
        correct_percentage: (completed > 0).then(|| correct as f64 * 100.0 / completed as f64),
    })
}

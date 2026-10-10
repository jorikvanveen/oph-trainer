use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261010_000001_accounts_and_progress"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .col(
                        ColumnDef::new(Accounts::Key)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CompletedQuizzes::Table)
                    .col(pk_auto(CompletedQuizzes::Id))
                    .col(
                        ColumnDef::new(CompletedQuizzes::AccountKey)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CompletedQuizzes::QuizId).text().not_null())
                    .index(
                        Index::create()
                            .name("completed_quizzes_account_quiz")
                            .col(CompletedQuizzes::AccountKey)
                            .col(CompletedQuizzes::QuizId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CompletedQuizzes::Table, CompletedQuizzes::AccountKey)
                            .to(Accounts::Table, Accounts::Key)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(QuestionOutcomes::Table)
                    .col(
                        ColumnDef::new(QuestionOutcomes::CompletedQuizId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestionOutcomes::Position)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QuestionOutcomes::QuestionId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        boolean(QuestionOutcomes::Correct)
                            .check(Expr::col(QuestionOutcomes::Correct).is_in([false, true])),
                    )
                    .primary_key(
                        Index::create()
                            .col(QuestionOutcomes::CompletedQuizId)
                            .col(QuestionOutcomes::QuestionId),
                    )
                    .index(
                        Index::create()
                            .name("question_outcomes_quiz_position")
                            .col(QuestionOutcomes::CompletedQuizId)
                            .col(QuestionOutcomes::Position)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(QuestionOutcomes::Table, QuestionOutcomes::CompletedQuizId)
                            .to(CompletedQuizzes::Table, CompletedQuizzes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuestionOutcomes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CompletedQuizzes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Key,
}

#[derive(DeriveIden)]
enum CompletedQuizzes {
    Table,
    Id,
    AccountKey,
    QuizId,
}

#[derive(DeriveIden)]
enum QuestionOutcomes {
    Table,
    CompletedQuizId,
    Position,
    QuestionId,
    Correct,
}

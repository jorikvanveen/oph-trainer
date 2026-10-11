use sea_orm_migration::{prelude::*, schema::boolean};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_192845_remove_quiz_display_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    fn use_transaction(&self) -> Option<bool> {
        Some(true)
    }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        replace_outcomes(manager, false).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("completed_quizzes_account_completion")
                    .table(CompletedQuizzes::Table)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        replace_outcomes(manager, true).await?;
        manager
            .create_index(
                Index::create()
                    .name("completed_quizzes_account_completion")
                    .table(CompletedQuizzes::Table)
                    .col(CompletedQuizzes::AccountKey)
                    .col(CompletedQuizzes::Id)
                    .to_owned(),
            )
            .await
    }
}

async fn replace_outcomes(
    manager: &SchemaManager<'_>,
    restore_positions: bool,
) -> Result<(), DbErr> {
    // SQLite cannot drop a column used by a table-level unique constraint.
    let mut table = Table::create();
    table
        .table(QuestionOutcomes::Replacement)
        .col(
            ColumnDef::new(QuestionOutcomes::CompletedQuizId)
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
        .foreign_key(
            ForeignKey::create()
                .from(
                    QuestionOutcomes::Replacement,
                    QuestionOutcomes::CompletedQuizId,
                )
                .to(CompletedQuizzes::Table, CompletedQuizzes::Id)
                .on_delete(ForeignKeyAction::Cascade),
        );
    let mut columns = vec![
        QuestionOutcomes::CompletedQuizId,
        QuestionOutcomes::QuestionId,
        QuestionOutcomes::Correct,
    ];
    let mut select = Query::select();
    select
        .columns(columns.clone())
        .from(QuestionOutcomes::Table);
    if restore_positions {
        table
            .col(
                ColumnDef::new(QuestionOutcomes::Position)
                    .integer()
                    .not_null(),
            )
            .index(
                Index::create()
                    .name("question_outcomes_quiz_position")
                    .col(QuestionOutcomes::CompletedQuizId)
                    .col(QuestionOutcomes::Position)
                    .unique(),
            );
        columns.push(QuestionOutcomes::Position);
        // Original presentation order is gone; rollback restores a stable question-ID order.
        select.expr_window_as(
            Func::cust(Alias::new("ROW_NUMBER")),
            WindowStatement::partition_by(QuestionOutcomes::CompletedQuizId)
                .order_by(QuestionOutcomes::QuestionId, Order::Asc)
                .to_owned(),
            QuestionOutcomes::Position,
        );
    }
    manager.create_table(table.to_owned()).await?;
    let mut copy = Query::insert();
    copy.into_table(QuestionOutcomes::Replacement)
        .columns(columns)
        .select_from(select)
        .map_err(|error| DbErr::Custom(error.to_string()))?;
    manager.get_connection().execute(&copy).await?;
    manager
        .drop_table(Table::drop().table(QuestionOutcomes::Table).to_owned())
        .await?;
    manager
        .rename_table(
            Table::rename()
                .table(QuestionOutcomes::Replacement, QuestionOutcomes::Table)
                .to_owned(),
        )
        .await
}

#[derive(DeriveIden, Clone)]
enum QuestionOutcomes {
    Table,
    #[sea_orm(iden = "question_outcomes_replacement")]
    Replacement,
    CompletedQuizId,
    QuestionId,
    Correct,
    Position,
}

#[derive(DeriveIden)]
enum CompletedQuizzes {
    Table,
    Id,
    AccountKey,
}

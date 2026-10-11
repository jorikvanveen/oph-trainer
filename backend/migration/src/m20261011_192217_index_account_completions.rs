use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_192217_index_account_completions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("completed_quizzes_account_completion")
                    .table(CompletedQuizzes::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CompletedQuizzes {
    Table,
    AccountKey,
    Id,
}

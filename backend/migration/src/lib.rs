pub use sea_orm_migration::prelude::*;

mod m20261010_000001_accounts_and_progress;
mod m20261011_192217_index_account_completions;
mod m20261011_192845_remove_quiz_display_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261010_000001_accounts_and_progress::Migration),
            Box::new(m20261011_192217_index_account_completions::Migration),
            Box::new(m20261011_192845_remove_quiz_display_data::Migration),
        ]
    }
}

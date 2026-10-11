use std::net::SocketAddr;

use axum::Router;
use migration::{Migrator, MigratorTrait};
use oph_trainer_backend::{
    accounts::AccountService, config::Config, grading::GradingService, practice::PracticeService,
    question_bank::QuestionBank, routes,
};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("warn,oph_trainer_backend=info")),
        )
        .init();

    let config = Config::load()?;
    tracing::info!(
        automatic_grading = config.openrouter_api_key.is_some(),
        use_x_forwarded_for = config.use_x_forwarded_for,
        static_dir = %config.static_dir,
        "Starting unofficial Orpheus interview trainer"
    );
    if config.openrouter_api_key.is_none() {
        tracing::warn!(
            "OPH_OPENROUTER_API_KEY is not set or is blank; automatic explanation grading is disabled"
        );
    }
    let grading = GradingService::new(config.openrouter_api_key.clone())?;
    let mut options = ConnectOptions::new(config.database_url);
    options.sqlx_logging(false);
    options.record_stmt_in_spans(false);
    tracing::info!("Connecting to database");
    let db = Database::connect(options).await?;
    tracing::info!("Applying pending database migrations");
    Migrator::up(&db, None).await?;
    tracing::info!("Database ready");

    let bank = QuestionBank::questions();
    tracing::info!(
        topics = bank.topics.len(),
        questions = bank.questions.len(),
        "Question bank loaded"
    );

    let api = routes::api_with_grading(
        PracticeService::new(bank),
        AccountService::new(db),
        grading,
        config.use_x_forwarded_for,
    );
    // A service mount reserves both "/api" and "/api/" from the static fallback.
    let app = Router::new()
        .nest_service("/api", api)
        .fallback_service(ServeDir::new(config.static_dir));
    let listener = TcpListener::bind(&config.bind_address).await?;
    tracing::info!(address = %listener.local_addr()?, "Listening");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

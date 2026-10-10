use axum::Router;
use migration::{Migrator, MigratorTrait};
use oph_trainer_backend::{
    config::Config, practice::PracticeService, question_bank::QuestionBank, routes,
};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::load()?;
    let mut options = ConnectOptions::new(config.database_url);
    options.sqlx_logging(false);
    let db = Database::connect(options).await?;
    Migrator::up(&db, None).await?;

    // A service mount reserves both "/api" and "/api/" from the static fallback.
    let practice = PracticeService::new(QuestionBank::default());
    let api = routes::api(practice);
    let app = Router::new()
        .nest_service("/api", api)
        .fallback_service(ServeDir::new(config.static_dir));
    let listener = TcpListener::bind(&config.bind_address).await?;
    tracing::info!(address = %listener.local_addr()?, "Listening");
    axum::serve(listener, app).await?;

    Ok(())
}

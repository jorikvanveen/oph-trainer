use axum::{Router, routing::get};
use migration::{Migrator, MigratorTrait};
use oph_trainer_backend::{config::Config, routes};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
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

    let app = Router::new()
        .route("/api/sample", get(routes::sample))
        .with_state(db);
    let listener = TcpListener::bind(&config.bind_address).await?;
    tracing::info!(address = %listener.local_addr()?, "Listening");
    axum::serve(listener, app).await?;

    Ok(())
}

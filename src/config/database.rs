use std::str::FromStr;
use std::time::Duration;
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};

pub async fn database_connection() -> PgPool {
    tracing::debug!("Setting up database connection");
    let db_url = dotenv::var("DATABASE_URL").expect("Failed to get database url from env");

    let options = PgConnectOptions::from_str(&db_url)
        .expect("failed to parse url")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("failed to connect to the database");

    tracing::debug!("Successfully connected");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    tracing::debug!("Successfully migrated");

    pg_pool
}
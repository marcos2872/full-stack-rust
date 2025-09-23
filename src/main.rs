use tokio::net::TcpListener;
use full_stack_rust::{
    config::{logs, database},
    routes::routers,
models::app::AppState};

#[tokio::main]

async fn main() {
    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.expect("bind failed");

    logs::logging();
    let db_poll = database::database_connection().await;

    let app_state = AppState {
        db: db_poll
    };

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let app = routers(app_state);

    axum::serve(listener, app).await.expect("running server failed");
}

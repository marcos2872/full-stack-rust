use tokio::net::TcpListener;
use full_stack_rust::config::{logs, database};
use full_stack_rust::routes::routers;

#[tokio::main]

async fn main() {
    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.expect("bind failed");

    logs::logging();
    database::database_connection().await;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let app = routers();

    axum::serve(listener, app).await.expect("running server failed");
}

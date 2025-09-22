use std::time::Duration;
use axum::{body::Body, http::Request, routing::{get}, Router};
use axum::response::{IntoResponse, Response};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Span;
use crate::handlers::{
    auth::{log_in, sing_up},
    {public::home},
    {todos::{creates, todos}}
};
use crate::handlers::auth::post_sing_up;

pub fn routers() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/healthcheck", get(healthcheck))
        .route("/log-in", get(log_in))
        .route("/create", get(creates))
        .route("/todos", get(todos))
        .route("/sign-up", get(sing_up).post(post_sing_up))
        .nest_service("/static", server_dir)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| tracing::info_span!("http-server"))
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failure));

    app
}

async fn healthcheck() -> Response {
    "server working".into_response()
}

fn on_request(request: &Request<Body>, _:&Span) {
    tracing::info!(
        "-> Request from {} {}",
        request.method(),
        request.uri()
    )
}
fn on_response(response: &Response<Body>, latency:Duration, _: &Span) {
    tracing::info!(
        "<- Response: status={} latency={:?}",response.status(), latency
    )
}
fn on_failure(error: ServerErrorsFailureClass, latency:Duration, _: &Span) {
    tracing::error!(
        "-X- ServerError: status={:?}, latency={:?}",error, latency
    )
}

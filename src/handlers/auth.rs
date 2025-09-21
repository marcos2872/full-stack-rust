use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{LogInTemplate, SingUpTemplate};
use askama::Template;
pub async fn log_in() -> Response {
    let html = LogInTemplate{}.render().unwrap();

    Html(html).into_response()
}

pub async fn sing_up() -> Response {
    let html = SingUpTemplate{}.render().unwrap();

    Html(html).into_response()
}

use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{HomeTemplate};
use askama::Template;

pub async fn home() -> Response {
    let html = HomeTemplate{}.render().unwrap();

    Html(html).into_response()
}

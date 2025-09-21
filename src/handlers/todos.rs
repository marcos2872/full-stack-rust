use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{CreatesTemplate, TodosTemplate};
use askama::Template;

pub async fn creates() -> Response {
    let html = CreatesTemplate{}.render().unwrap();

    Html(html).into_response()
}

pub async fn todos() -> Response {
    let html = TodosTemplate{}.render().unwrap();

    Html(html).into_response()
}


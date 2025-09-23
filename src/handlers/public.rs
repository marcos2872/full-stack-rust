use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{HomeTemplate};
use askama::Template;
use crate::handlers::errors::AppError;

pub async fn home() -> Result<Response, AppError> {
    let html = HomeTemplate{}.render()?;

    Ok(Html(html).into_response())
}

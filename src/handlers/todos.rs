use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{CreatesTemplate, TodosTemplate};
use askama::Template;
use crate::handlers::errors::AppError;

pub async fn creates() -> Result<Response, AppError> {
    let html = CreatesTemplate{}.render()?;

    Ok(Html(html).into_response())
}

pub async fn todos() -> Result<Response, AppError> {
    let html = TodosTemplate{}.render()?;

    Ok(Html(html).into_response())
}


use askama::Template;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse};
use thiserror::Error;
use crate::data::errors::DataError;
use crate::models::templates;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body>{
        let (status , response) = match self {
            AppError::Database(e) => server_error(e.to_string()),
            AppError::Template(e) => server_error(e.to_string()),
        };

        (status, response).into_response()
    }
}

fn server_error(e:String) -> (StatusCode, Response<Body>) {
    tracing::error!("Server error: {}", e);

    let html = templates::ServerErrorTemplate {}.render().unwrap();

    // match html {
    //     Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html).into_response()),
    //     Err(e) => {
    //         tracing::error!("Server error: {}", e.to_string());
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Internal server error".into_response()
    //             )
    //     }
    // }

    (StatusCode::INTERNAL_SERVER_ERROR, Html(html).into_response())
}

use crate::models::{
    templates::{LogInTemplate, SingUpTemplate},
    user_form_model::AuthFormModel
};
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
    http::StatusCode
};
use validator::Validate;
use crate::utils::extract_error::extract_error;
use crate::data::{user, errors::DataError};
use crate::handlers::errors::AppError;
use crate::models::app::AppState;

pub async fn log_in() -> Result<Response, AppError> {
    let html = LogInTemplate{}.render()?;

    Ok(Html(html).into_response())
}

pub async fn sing_up() -> Result<Response, AppError> {
    let html = SingUpTemplate{email: "", email_error: "", password_error: ""}.render()?;

    Ok(Html(html).into_response())
}

pub async fn post_sing_up(State(app_state): State<AppState>, Form(user_form): Form<AuthFormModel>) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let response = user::create_user(
                &app_state.db,
                &user_form.email,
                &user_form.password).await;

            if let Err(err) = response {
                if let DataError::FailedQuery(e) = err {
                    tracing::error!("Failed to create user: {}", e);

                    return Ok(Redirect::to("/sign-up").into_response())
                } else {
                    Err(err)?
                }
            }
            Ok(Redirect::to("/").into_response())
        }
        Err(e) => {

            let err = e.to_string();

            let mut email_error = String::new();
            let mut password_error = String::new();

            extract_error(&err, |field, message | {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message;
                }
            });

            let html_string = SingUpTemplate {
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
            }.render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())

        }
    }
}
 
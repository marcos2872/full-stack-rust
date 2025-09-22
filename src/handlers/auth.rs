use axum::response::{Html, IntoResponse, Redirect, Response};
use crate::models::{
    templates::{LogInTemplate, SingUpTemplate},
    user_form_model::AuthFormModel
};
use askama::Template;
use axum::Form;
use axum::http::StatusCode;
use validator::Validate;
use crate::utils::extract_error::extract_error;

pub async fn log_in() -> Response {
    let html = LogInTemplate{}.render().unwrap();

    Html(html).into_response()
}

pub async fn sing_up() -> Response {
    let html = SingUpTemplate{email: "", email_error: "", password_error: ""}.render().unwrap();

    Html(html).into_response()
}

pub async fn post_sing_up(Form(user_form): Form<AuthFormModel>) -> Response {
    match user_form.validate() {
        Ok(_) => {
            Redirect::to("/").into_response()
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
            }.render().unwrap();

            let response = Html(html_string).into_response();

            (StatusCode::BAD_REQUEST, response).into_response()

        }
    }
}
 
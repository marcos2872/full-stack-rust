use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LogInTemplate {
}

#[derive(Template)]
#[template(path = "pages/creates.html")]
pub struct CreatesTemplate {
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate {
}

#[derive(Template)]
#[template(path = "pages/sing-up.html")]
pub struct SingUpTemplate<'t> {
    pub email: &'t str,
    pub email_error: &'t str,
    pub password_error: &'t str,
}

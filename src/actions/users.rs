use axum::{response::Html, Form};
use serde::Deserialize;

pub async fn new() -> Html<&'static str> {
    let template = r#"
      <h1>Register</h1>
      <form method="post" enctype="application/x-www-form-urlencoded">
        <input type="text" name="email" />
        <input type="text" name="password" />
        <input type="text" name="password_confirmation" />
        // <button type="submit">Create account</button>
      </form>
    "#;

    Html(template)
}

#[derive(Deserialize, Clone)]
pub struct Register {
    email: String,
    password: String,
    password_confirmation: String,
}

pub async fn create(Form(form): Form<Register>) -> Html<&'static str> {
    Html("User submitted")
}

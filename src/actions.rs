use actix_web::{get, web, Responder, Result};
use serde::Serialize;

pub mod ocr;

#[derive(Serialize)]
struct Health<'a> {
    status: &'a str,
}

#[get("/")]
pub async fn root() -> Result<impl Responder> {
    let result = Health { status: "OK" };

    Ok(web::Json(result))
}

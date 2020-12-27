use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::CONTENT_TYPE;
use serde::Serialize;

#[derive(Serialize)]
struct StatusResponse {
    message: &'static str
}

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .set_header(CONTENT_TYPE, "application/json")
        .json(StatusResponse { message: "API is fully operational!" })
}

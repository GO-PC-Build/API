use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::CONTENT_TYPE;

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .set_header(CONTENT_TYPE, "application/json")
        .json(crate::types::status::StatusResponse {
            message: "API is fully operational!"
        })
}

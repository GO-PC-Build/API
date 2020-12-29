use actix_web::{get, Responder};

use crate::types::status::StatusResponse;
use crate::utils::response::ok;

#[get("/status")]
pub async fn status() -> impl Responder {
    ok(StatusResponse {
        message: "API is fully operational!"
    })
}

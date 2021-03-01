use actix_web::{get, post, web::Json};
use actix_web::Responder;

use crate::types::reserve::{ReserveRequest, ReserveResponse};
use crate::utils::postgres::{get_schemes, is_valid_code, make_reservation};
use crate::utils::response::{bad_request_message, ok};

#[post("/reserve")]
pub async fn reserve(body: Json<ReserveRequest>) -> impl Responder {
    if is_valid_code(body.code).await {
        match make_reservation(body.workshop, body.user_id, body.location, body.code).await {
            Ok(data) => ok(ReserveResponse { success: &data == "Success" }),
            Err(e) => e
        }
    } else {
        bad_request_message("An invalid code was provided".to_string())
    }
}

#[get("/schemes")]
pub async fn schemes() -> impl Responder {
    get_schemes().await
}

use actix_web::{get, HttpRequest, Responder};

use crate::utils::postgres::get_user;
use crate::utils::response::{internal_server_error, no_auth_header};

#[get("/@me")]
pub async fn me(req: HttpRequest) -> impl Responder {
    match &req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => get_user(token),
            Err(_) => internal_server_error("Could not convert token...")
        }
        None => no_auth_header()
    }
}

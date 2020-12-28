use actix_web::{get, Responder, HttpRequest};
use crate::utils::response::{no_auth_header, internal_server_error};
use crate::utils::postgres::get_user;

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
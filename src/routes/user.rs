use actix_web::{get, HttpRequest, post, Responder};
use actix_web::web::{Json, Path};

use crate::types::user::UserThirdPlatformValueParam;
use crate::utils::postgres::{connect_third_party, get_user, get_student};
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

#[post("/connect/{platform}")]
pub async fn connect(req: HttpRequest, platform: Path<String>, data: Json<UserThirdPlatformValueParam>) -> impl Responder {
    match &req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => connect_third_party(token, &platform, &data.value),
            Err(_) => internal_server_error("Could not convert token...")
        }
        None => no_auth_header()
    }
}

#[get("/id/{id}")]
pub async fn user(id: Path<i32>) -> impl Responder {
    get_student(id.clone()).await
}

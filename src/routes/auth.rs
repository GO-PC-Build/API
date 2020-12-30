use actix_web::{HttpRequest, post, Responder, web::Json};

use crate::types::auth::{SignInRequest, SignUpRequest, TokenResponse};
use crate::types::status::StatusResponse;
use crate::utils::postgres::{create_account, get_login_token, revoke_token, get_user_token_from_linked};
use crate::utils::response::{internal_server_error, no_auth_header, ok};
use crate::types::user::UserThirdPlatformValueParam;

#[post("/login")]
pub async fn login(data: Json<SignInRequest>) -> impl Responder {
    match get_login_token(&data) {
        Ok(token) => ok(TokenResponse { token }),
        Err(e) => e
    }
}

#[post("/extern/login")]
pub async fn extern_login(data: Json<UserThirdPlatformValueParam>) -> impl Responder {
    match get_user_token_from_linked(&data.value, &data.token).await {
        Ok(token) => ok(TokenResponse { token }),
        Err(e) => e
    }
}

#[post("/register")]
pub async fn register(data: Json<SignUpRequest>) -> impl Responder {
    match create_account(&data) {
        Ok(token) => ok(TokenResponse { token }),
        Err(e) => e
    }
}

#[post("/revoke")]
pub async fn revoke(req: HttpRequest) -> impl Responder {
    match &req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => match revoke_token(token) {
                Ok(message) => ok(StatusResponse { message }),
                Err(e) => e
            }
            Err(_) => internal_server_error("Could not convert token...")
        }
        None => no_auth_header()
    }
}

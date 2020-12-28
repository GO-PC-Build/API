use actix_web::{HttpResponse, post, Responder, web::Json, HttpRequest};

use crate::types::auth::{SignInRequest, SignUpRequest, TokenResponse};
use crate::utils::postgres::{get_login_token, create_account};
use crate::utils::response::{ok, bad_request_message};
use crate::utils::auth::is_valid_request;

#[post("/login")]
pub async fn login(data: Json<SignInRequest>) -> impl Responder {
    match get_login_token(&data) {
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
    if !is_valid_request(&req) {
        return bad_request_message("No 'Authorization' header was present on the request")
    }
    HttpResponse::Ok().body("Revoke route")
}

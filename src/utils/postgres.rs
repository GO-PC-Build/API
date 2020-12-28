use actix_web::HttpResponse;

use crate::types::auth::{SignInRequest, SignUpRequest};
use crate::utils::response::{internal_server_error_message, ok};
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use crate::types::user::User;

pub fn get_login_token(data: &SignInRequest) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint"))
}

pub fn create_account(data: &SignUpRequest) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint"))
}

pub fn revoke_token(token: &str) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint"))
}

pub fn get_user(token: &str) -> HttpResponse {
    // let created_at = SystemTime::now();
    // let date: DateTime<Utc> = created_at.into();

    // ok(User {
    //     id: "123example321".to_string(),
    //     nickname: "example lord".to_string(),
    //     email: "example@example.com".to_string(),
    //     avatar: "http://cdn.example.com/pfp/123example321".to_string(),
    //     date: date.to_rfc3339()
    // })
    internal_server_error_message("Unimplemented endpoint")
}

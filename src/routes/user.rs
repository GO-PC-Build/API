use actix_web::{get, Responder, HttpRequest};
use crate::utils::response::{ok, no_auth_header};
use crate::utils::auth::is_valid_request;
use crate::types::user::User;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

#[get("/@me")]
pub async fn me(req: HttpRequest) -> impl Responder {
    if !is_valid_request(&req) {
        return no_auth_header()
    }

    let created_at = SystemTime::now();
    let date: DateTime<Utc> = created_at.into();

    ok(User {
        id: "123example321".to_string(),
        nickname: "example lord".to_string(),
        email: "example@example.com".to_string(),
        avatar: "http://cdn.example.com/pfp/123example321".to_string(),
        date: date.to_rfc3339()
    })
}
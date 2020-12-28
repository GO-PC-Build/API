use std::time::SystemTime;

use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub date: String,
}

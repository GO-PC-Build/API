use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct UserThirdPlatformValueParam {
    pub value: String,
    pub token: Option<String>
}

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub date: String,
}

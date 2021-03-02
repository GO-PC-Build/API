use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReserveRequest {
    pub internalnr: i32,
    pub class: String,
    pub firstname: String,
    pub lastname: String,
    pub workshop: i32,
    pub location: i32,
    pub code: i32,
}

#[derive(Serialize)]
pub struct ReserveResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct SchemeResponse {
    pub schemes: Vec<Vec<i32>>,
}

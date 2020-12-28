use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: &'static str
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String
}

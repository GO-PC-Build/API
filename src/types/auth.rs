use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

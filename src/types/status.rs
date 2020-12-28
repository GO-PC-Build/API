use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub message: &'static str
}
use serde::{Serialize};

#[derive(Serialize)]
pub struct BaseException {
    pub message: &'static str,
    pub error: &'static str
}

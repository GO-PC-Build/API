use actix_web::HttpResponse;

use crate::types::auth::{SignInRequest, SignUpRequest};
use crate::utils::response::internal_server_error_message;

pub fn get_login_token(data: &SignInRequest) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint"))
}

pub fn create_account(data: &SignUpRequest) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint"))
}

// pub fn revoke_token()

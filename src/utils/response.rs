use actix_web::http::header::CONTENT_TYPE;
use actix_web::HttpResponse;
use serde::Serialize;
use actix_web::dev::HttpResponseBuilder;
use crate::types::exceptions::BaseException;

pub fn json_former<T: Serialize>(mut res: HttpResponseBuilder, data: T) -> HttpResponse {
    res.set_header(CONTENT_TYPE, "application/json").json(data)
}

pub fn ok<T: Serialize>(value: T) -> HttpResponse {
    json_former(HttpResponse::Ok(), value)
}

pub fn bad_request<T: Serialize>(value: T) -> HttpResponse {
    json_former(HttpResponse::BadRequest(), value)
}

pub fn bad_request_message(error: &'static str) -> HttpResponse {
    bad_request(BaseException {
        message: "Oops... You did something wrong! (See error for more information)",
        error
    })
}

pub fn no_auth_header() -> HttpResponse {
    bad_request_message("No or an invalid 'Authorization' header was present on the request")
}

pub fn internal_server_error<T: Serialize>(value: T) -> HttpResponse {
    json_former(HttpResponse::InternalServerError(), value)
}

pub fn internal_server_error_message(error: &'static str) -> HttpResponse {
    internal_server_error(BaseException {
        message: "Oops... Something went wrong on our side.\n\
        If this keeps happening please contact the developer on discord. (Arthur#0002)",
        error
    })
}

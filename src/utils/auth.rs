use actix_web::HttpRequest;

pub fn is_valid_request(req: &HttpRequest) -> bool {
    match req.headers().get("Authorization") {
        Some(_) => true,
        None => false
    }
}

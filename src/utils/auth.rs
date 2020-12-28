use actix_web::HttpRequest;

pub fn is_valid_request(req: &HttpRequest) -> Option<&str> {
    req.headers().get("Authorization")?.to_str().ok()
}

use actix_web::HttpRequest;

pub fn is_valid_request(req: &HttpRequest) -> bool {
    match req.headers().get("Authorization") {
        Some(token) => {
            // TODO: Check in DB if its a valid token
            true
        },
        None => false
    }
}

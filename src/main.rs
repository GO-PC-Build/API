use actix_web::{App, http, HttpServer};
use actix_cors::Cors;
use std::io::Result;

mod utils;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:25578";
    println!("Running a server on http://{}", addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            // TODO: FOR FINAL MERGE, SET CORRECT CORS ORIGIN
            // .allowed_origin("eco.xiler.net")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(routes::status::status)
    }).bind(addr)?.run().await
}

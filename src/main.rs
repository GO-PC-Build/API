extern crate chrono;
extern crate dotenv;

use std::io::Result;

use actix_cors::Cors;
use actix_web::{App, http, HttpServer, web};

mod utils;
mod routes;
mod types;

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
            .service(web::scope("/auth")
                .service(routes::auth::login)
                .service(routes::auth::extern_login)
                .service(routes::auth::register)
                .service(routes::auth::revoke))
            .service(web::scope("/user")
                .service(routes::user::me)
                .service(routes::user::connect))
    }).bind(addr)?.run().await
}

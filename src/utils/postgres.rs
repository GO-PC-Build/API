use std::env;

use actix_web::HttpResponse;
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use uuid::Uuid;

use crate::types::auth::{SignInRequest, SignUpRequest};
use crate::utils::response::{bad_request, internal_server_error_message};

pub fn connect() -> Result<Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file!");

    Client::connect(&database_url, NoTls)
}

pub fn get_user_uuid(token: String) -> Result<Uuid, String> {
    match connect() {
        Ok(mut client) => match client.query("SELECT acc FROM tokens WHERE token = $1", &[&token]) {
            Ok(data) => {
                let mut uuid: Option<Uuid> = None;
                for row in data {
                    let id: Uuid = row.get(0);
                    uuid = Some(id);
                }

                match uuid {
                    Some(uuid) => Ok(uuid),
                    None => Err("No associated account found for the token.".to_string())
                }
            }
            Err(e) => Err(format!("Could not execute query. {}", e))
        }
        Err(e) => Err(format!("Couldn't connect to DB. {}", e))
    }
}

pub fn get_user_token(user: Uuid) -> Result<String, String> {
    match connect() {
        Ok(mut client) => match client.query("SELECT token FROM tokens WHERE acc = $1", &[&user]) {
            Ok(data) => {
                let mut token: Option<String> = None;
                for row in data {
                    let t: String = row.get(0);
                    token = Some(t);
                }

                match token {
                    Some(token) => Ok(token),
                    None => Err("Could not properly iterate through records.".to_string())
                }
            }
            Err(e) => Err(format!("Could not execute query. {}", e))
        }
        Err(e) => Err(format!("Couldn't connect to DB. {}", e))
    }
}

pub fn get_login_token(data: &SignInRequest) -> Result<String, HttpResponse> {
    match connect() {
        Ok(mut client) => match client.query(
            "SELECT id \
            FROM accounts \
            WHERE (nickname ILIKE $1 OR email ILIKE $1)\
              AND password = $2;", &[&data.username, &data.password]) {
            Ok(data) => {
                if data.len() >= 1 {
                    let mut token: Result<String, HttpResponse> =
                        Err(internal_server_error_message("Something went wrong while trying to iterate through the records.".to_string()));
                    for row in data {
                        let id: Uuid = row.get(0);
                        token = match get_user_token(id) {
                            Ok(token) => Ok(token),
                            Err(e) => Err(internal_server_error_message(e))
                        }
                    }
                    return token
                }
                Err(bad_request("Invalid credentials provided."))
            }
            Err(e) => Err(internal_server_error_message(format!("Couldn't execute query. {}", e)))
        }
        Err(e) => Err(internal_server_error_message(format!("Couldn't connect to DB. {}", e)))
    }
}

pub fn create_account(data: &SignUpRequest) -> Result<String, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint".to_string()))
}

pub fn revoke_token(token: &str) -> Result<&'static str, HttpResponse> {
    // TODO: IMPLEMENT ENDPOINT
    Err(internal_server_error_message("Unimplemented endpoint".to_string()))
}

pub fn get_user(token: &str) -> HttpResponse {
    // TODO: IMPLEMENT ENDPOINT
    // let created_at = SystemTime::now();
    // let date: DateTime<Utc> = created_at.into();

    // ok(User {
    //     id: "123example321".to_string(),
    //     nickname: "example lord".to_string(),
    //     email: "example@example.com".to_string(),
    //     avatar: "http://cdn.example.com/pfp/123example321".to_string(),
    //     date: date.to_rfc3339()
    // })
    internal_server_error_message("Unimplemented endpoint".to_string())
}

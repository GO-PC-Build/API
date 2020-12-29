use std::env;
use std::time::SystemTime;

use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use uuid::Uuid;

use crate::types::auth::{SignInRequest, SignUpRequest};
use crate::types::exceptions::BaseException;
use crate::types::user::User;
use crate::utils::response::{bad_request, internal_server_error_message, ok};

pub fn connect() -> Result<Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file!");

    Client::connect(&database_url, NoTls)
}

pub fn get_user_uuid(token: String) -> Result<Uuid, String> {
    match connect() {
        Ok(mut client) => match client.query("SELECT acc FROM tokens WHERE token = $1;", &[&token]) {
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
        Ok(mut client) => match client.query("SELECT token FROM tokens WHERE acc = $1;", &[&user]) {
            Ok(data) => {
                if data.len() == 0 {
                    return match client.query("CALL create_token($1);", &[&user]) {
                        Ok(_) => get_user_token(user),
                        Err(e) => Err(format!("Could not execute query. {}", e))
                    }
                }

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
                    return token;
                }
                Err(bad_request(BaseException {
                    message: "Invalid credentials provided.",
                    error: "".to_string()
                }))
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
    match connect() {
        Ok(mut client) => match client.execute("DELETE FROM tokens WHERE token = $1;", &[&token]) {
            Ok(records) => {
                if records == 0 {
                    return Err(bad_request(BaseException {
                        message: "An invalid authorization token was provided.",
                        error: "".to_string()
                    }))
                }
                Ok("Successfully removed the token.")
            }
            Err(e) => Err(internal_server_error_message(format!("Couldn't execute query. {}", e)))
        }
        Err(e) => Err(internal_server_error_message(format!("Couldn't connect to DB. {}", e)))
    }
}

pub fn get_user(token: &str) -> HttpResponse {
    match get_user_uuid(token.parse().unwrap()) {
        Ok(id) => match connect() {
            Ok(mut client) => match client.query(
                "SELECT nickname, email, avatar, date \
                FROM accounts \
                WHERE id = $1;", &[&id]) {
                Ok(data) => {
                    let mut user: Option<User> = None;
                    for row in data {
                        let nickname: String = row.get(0);
                        let email: String = row.get(1);
                        let avatar: String = row.get(2);
                        let created_at: SystemTime = row.get(3);
                        let date: DateTime<Utc> = created_at.into();
                        user = Some(User { id, nickname, email, avatar, date: date.to_rfc3339() });
                    }
                    match user {
                        Some(user) => ok(user),
                        None => bad_request("Could not find user associated with token")
                    }
                }
                Err(e) => internal_server_error_message(format!("Couldn't execute query. {}", e))
            }
            Err(e) => internal_server_error_message(format!("Couldn't connect to DB. {}", e))
        }
        Err(e) => internal_server_error_message(e)
    }
}

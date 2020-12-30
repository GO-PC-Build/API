use std::env;
use std::time::SystemTime;

use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use postgres::error::SqlState;
use reqwest::{Client as ReqwestClient, StatusCode};
use serde_json::{Error as JsonError, Value};
use uuid::Uuid;

use crate::types::auth::{SignInRequest, SignUpRequest};
use crate::types::exceptions::BaseException;
use crate::types::status::StatusResponse;
use crate::types::user::User;
use crate::utils::response::{bad_request, bad_request_message, internal_server_error_message, ok};

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
                    };
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

pub fn get_avatar(email: String) -> String {
    let digest = md5::compute(email.as_bytes());
    format!("https://www.gravatar.com/avatar/{:?}?d=https://www.kindpng.com/picc/m/421-4212275_transparent-default-avatar-png-avatar-img-png-download.png", digest)
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
                    error: "".to_string(),
                }))
            }
            Err(e) => Err(internal_server_error_message(format!("Couldn't execute query. {}", e)))
        }
        Err(e) => Err(internal_server_error_message(format!("Couldn't connect to DB. {}", e)))
    }
}

pub fn create_account(data: &SignUpRequest) -> Result<String, HttpResponse> {
    match connect() {
        Ok(mut client) => match client.execute(
            "INSERT INTO accounts (nickname, email, avatar, password) \
            VALUES ($1, $2, $3, $4);", &[&data.username, &data.email, &get_avatar((&*data.email).to_string()), &data.password]) {
            Ok(records) => {
                if records == 0 {
                    return Err(internal_server_error_message("Something went wrong while trying to create account.".to_string()));
                }
                Ok("Successfully created account.".to_string())
            }
            Err(e) => {
                if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                    return Err(bad_request(BaseException {
                        message: "An account already exists with those field(s).",
                        error: e.to_string(),
                    }));
                }
                Err(internal_server_error_message(format!("Couldn't execute query. {}", e)))
            }
        }
        Err(e) => Err(internal_server_error_message(format!("Couldn't connect to DB. {}", e)))
    }
}

pub fn revoke_token(token: &str) -> Result<&'static str, HttpResponse> {
    match connect() {
        Ok(mut client) => match client.execute("DELETE FROM tokens WHERE token = $1;", &[&token]) {
            Ok(records) => {
                if records == 0 {
                    return Err(bad_request(BaseException {
                        message: "An invalid authorization token was provided.",
                        error: "".to_string(),
                    }));
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
                        None => bad_request_message("Could not find user associated with token".parse().unwrap())
                    }
                }
                Err(e) => internal_server_error_message(format!("Couldn't execute query. {}", e))
            }
            Err(e) => internal_server_error_message(format!("Couldn't connect to DB. {}", e))
        }
        Err(e) => internal_server_error_message(e)
    }
}

pub fn connect_third_party(token: &str, platform: &str, value: &str) -> HttpResponse {
    match get_user_uuid(token.parse().unwrap()) {
        Ok(id) => match connect() {
            Ok(mut client) => match client.execute(
                "INSERT INTO linked (acc, type, value) \
                VALUES ($1, (\
                SELECT id \
                FROM linked_types \
                WHERE name ILIKE $2\
                ), $3);", &[&id, &platform, &value]) {
                Ok(_) => ok(StatusResponse { message: "Successfully connected accounts!" }),
                Err(e) => {
                    if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                        return bad_request(BaseException {
                            message: "A linked account was already found.",
                            error: e.to_string(),
                        });
                    }
                    internal_server_error_message(format!("Couldn't execute query. {}", e))
                }
            }
            Err(e) => internal_server_error_message(format!("Couldn't connect to DB. {}", e))
        }
        Err(e) => internal_server_error_message(e)
    }
}

pub async fn is_valid_discord_token(token: &String, user: &String) -> bool {
    let client = ReqwestClient::new();
    let req = client.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", &token)).send();
    match req.await {
        Ok(res) => {
            if res.status() == StatusCode::UNAUTHORIZED {
                return false;
            }
            match res.text().await {
                Ok(data) => {
                    let v: Result<Value, JsonError> = serde_json::from_str(&data);
                    match v {
                        Ok(r) => {
                            // Remove string quotes:
                            let mut res = r["id"].to_string();
                            res.remove(0);
                            res.pop();

                            // Check if the requested ID is the same as the given:
                            &res.to_string() == user
                        }
                        Err(_) => false
                    }
                }
                Err(_) => false
            }
        }
        Err(_) => false
    }
}

pub fn get_external_user(user: Option<Uuid>) -> Result<String, HttpResponse> {
    match user {
        Some(user) => match get_user_token(user) {
            Ok(token) => Ok(token),
            Err(e) => Err(internal_server_error_message(e))
        }
        None => Err(bad_request_message("Could not find user associated with token".parse().unwrap()))
    }
}

pub async fn get_user_token_from_linked(value: &String, token: &Option<String>) -> Result<String, HttpResponse> {
    match connect() {
        Ok(mut client) => match client.query(
            "SELECT LOWER(name), acc \
            FROM linked, linked_types \
            WHERE value = $1 \
              AND type = id;", &[&value]) {
            Ok(data) => {
                let mut type_name: Option<String> = None;
                let mut user: Option<Uuid> = None;
                for row in data {
                    let name: String = row.get(0);
                    let acc: Uuid = row.get(1);
                    type_name = Some(name);
                    user = Some(acc);
                }

                match type_name {
                    Some(provided_token_type) => match &*provided_token_type {
                        "discord" => {
                            match &token {
                                Some(user_token) => {
                                    return if is_valid_discord_token(&user_token, &value).await {
                                        get_external_user(user)
                                    } else {
                                        Err(bad_request_message("An invalid token was provided.".to_string()))
                                    };
                                }
                                None => Err(bad_request_message("Failed to pass security measures.".to_string()))
                            }
                        }
                        _ => get_external_user(user)
                    },
                    None => Err(internal_server_error_message("Invalid token.".to_string()))
                }
            }
            Err(e) => Err(internal_server_error_message(format!("Couldn't execute query. {}", e))),
        }
        Err(e) => Err(internal_server_error_message(format!("Couldn't connect to DB. {}", e)))
    }
}

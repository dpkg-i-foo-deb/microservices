use std::{env, os};

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct JwtPair {
    auth_token: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    usr: String,
    exp: usize,
}

impl JwtPair {
    pub fn new(usr: &str) -> JwtPair {
        let auth_exp = generate_auth_expiration();

        let refresh_exp = generate_refresh_expiration();

        let auth_claims = Claims::new(usr, auth_exp as usize);

        let refresh_claims = Claims::new(usr, refresh_exp as usize);

        let auth_token = generate_token_string(&auth_claims);

        let refresh_token = generate_token_string(&refresh_claims);

        JwtPair {
            auth_token,
            refresh_token,
        }
    }

    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
}

impl Claims {
    fn new(usr: &str, exp: usize) -> Claims {
        Claims {
            usr: usr.to_string(),
            exp,
        }
    }
}

fn generate_token_string(claims: &Claims) -> String {
    let secret = fetch_secret();

    match encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Could not generate token {}", err);
            "ERROR".to_string()
        }
    }
}

fn fetch_secret() -> String {
    match env::var("SECRET") {
        Ok(secret) => secret,
        Err(err) => panic!("Could not retrieve the JWT secret {}", err),
    }
}

fn generate_auth_expiration() -> i64 {
    match Utc::now().checked_add_signed(chrono::Duration::minutes(15)) {
        Some(result) => result.timestamp(),
        None => 0,
    }
}

fn generate_refresh_expiration() -> i64 {
    match Utc::now().checked_add_signed(chrono::Duration::days(2)) {
        Some(result) => result.timestamp(),
        None => 0,
    }
}

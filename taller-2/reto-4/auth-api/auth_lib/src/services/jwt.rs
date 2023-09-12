use std::env;

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::CoreError;

#[derive(Serialize, Deserialize)]
pub struct Claims<'c> {
    exp: usize,
    role: &'c str,
    email: &'c str,
    tk_type: &'c str,
}

pub struct JWTService {}

impl JWTService {
    pub fn new() -> JWTService {
        JWTService {}
    }

    pub fn generate_access_tk(&self, role: &str, email: &str) -> Result<String, CoreError> {
        let exp = generate_auth_exp();

        let claims = Claims {
            exp,
            role,
            email,
            tk_type: "ACCESS",
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret().as_ref()),
        )?;

        Ok(tk)
    }

    pub fn generate_refresh_tk(&self, role: &str, email: &str) -> Result<String, CoreError> {
        let exp = generate_refresh_exp();

        let claims = Claims {
            exp,
            role,
            email,
            tk_type: "REFRESH",
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret().as_ref()),
        )?;

        Ok(tk)
    }
}

fn generate_auth_exp() -> usize {
    match Utc::now().checked_add_signed(chrono::Duration::minutes(15)) {
        Some(duration) => duration.timestamp() as usize,
        None => 0,
    }
}

fn generate_refresh_exp() -> usize {
    match Utc::now().checked_add_signed(chrono::Duration::days(3)) {
        Some(duration) => duration.timestamp() as usize,
        None => 0,
    }
}

fn get_secret() -> String {
    match env::var("SECRET") {
        Ok(secret) => secret,
        Err(err) => panic!("Secret key is required {}", err),
    }
}

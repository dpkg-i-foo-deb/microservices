use std::env;

use chrono::Utc;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    usr: String,
}

pub fn generate_jwt(usr: &str) -> String {
    let expiration = Utc::now().checked_add_signed(chrono::Duration::minutes(15));

    let expiration = match expiration {
        Some(result) => result.timestamp(),
        None => 0,
    };

    let claims = Claims {
        exp: expiration as usize,
        usr: String::from(usr),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(fetch_secret().as_bytes()),
    ) {
        Ok(t) => t,
        Err(err) => panic!("Could not generate the token! {}", err),
    };
    token
}

fn fetch_secret() -> String {
    match env::var("SECRET") {
        Ok(var) => var,
        Err(err) => panic!("Could not fetch the secret {err}"),
    }
}

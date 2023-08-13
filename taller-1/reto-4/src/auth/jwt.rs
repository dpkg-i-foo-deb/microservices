use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &str = "uwu";

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
        &EncodingKey::from_secret(SECRET.as_bytes()),
    ) {
        Ok(t) => t,
        Err(err) => panic!("Could not generate the token! {}", err),
    };
    token
}

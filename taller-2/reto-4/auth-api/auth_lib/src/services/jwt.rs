use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::CoreError;

#[derive(Serialize, Deserialize)]
pub struct Claims<'c> {
    exp: usize,
    role: &'c str,
    tk_type: &'c str,
}

#[derive(Clone)]
pub struct JWTService {
    secret: String,
}

impl JWTService {
    pub fn new(secret: String) -> JWTService {
        JWTService { secret }
    }

    pub fn generate_access_tk(&self, role: &str) -> Result<String, CoreError> {
        let exp = generate_auth_exp();

        let claims = Claims {
            exp,
            role,
            tk_type: "ACCESS",
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(tk)
    }

    pub fn generate_refresh_tk(&self, role: &str) -> Result<String, CoreError> {
        let exp = generate_refresh_exp();

        let claims = Claims {
            exp,
            role,
            tk_type: "REFRESH",
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
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

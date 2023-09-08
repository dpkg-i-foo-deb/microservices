use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::CoreError;

#[derive(Serialize, Deserialize)]
pub struct Claims<'c> {
    exp: usize,
    role: &'c str,
}

pub struct JWTService<'j> {
    secret: &'j str,
}

impl JWTService<'_> {
    pub fn new(secret: &str) -> JWTService {
        JWTService { secret }
    }

    pub fn generate_access_tk(&self, role: &str) -> Result<String, CoreError> {
        let exp = generate_auth_exp();

        let claims = Claims { exp, role };

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

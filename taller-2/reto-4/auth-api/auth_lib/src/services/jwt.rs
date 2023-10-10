use std::env;

use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::CoreError;

use super::user::UserService;

const AUTH: &str = "AUTH";
const REFRESH: &str = "REFRESH";
const PASSWD_RECOVER: &str = "PASSWD_RECOVER";

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    role: String,
    email: String,
    tk_type: String,
}

pub struct JWTService {
    user_service: UserService,
}

impl JWTService {
    pub fn new(user_service: UserService) -> JWTService {
        JWTService { user_service }
    }

    pub fn generate_tk(&self, tk_type: &str, email: &str, role: &str) -> Result<String, CoreError> {
        match tk_type {
            AUTH => Ok(self.generate_access_tk(role, email)?),
            REFRESH => Ok(self.generate_refresh_tk(role, email)?),
            PASSWD_RECOVER => Ok(self.generate_passwd_recover_tk(role, email)?),
            _ => Err(CoreError::JWTTypeError("On generation attempt")),
        }
    }

    pub fn generate_access_tk(&self, role: &str, email: &str) -> Result<String, CoreError> {
        let exp = generate_auth_exp();

        let claims = Claims {
            exp,
            role: role.to_owned(),
            email: email.to_owned(),
            tk_type: AUTH.to_owned(),
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
            role: role.to_owned(),
            email: email.to_owned(),
            tk_type: REFRESH.to_owned(),
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret().as_ref()),
        )?;

        Ok(tk)
    }

    pub fn generate_passwd_recover_tk(&self, role: &str, email: &str) -> Result<String, CoreError> {
        self.user_service.find_by_email(email)?;

        let exp = generate_passwd_recover_exp();

        let claims = Claims {
            exp,
            role: role.to_owned(),
            email: email.to_owned(),
            tk_type: PASSWD_RECOVER.to_owned(),
        };

        let tk = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_secret().as_ref()),
        )?;

        Ok(tk)
    }

    pub fn check_passwd_recover_tk(&self, tk: &str) -> Result<(), CoreError> {
        let claims = decode::<Claims>(
            tk,
            &DecodingKey::from_secret(get_secret().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match claims {
            Ok(data) => match data.claims.tk_type.as_str() {
                PASSWD_RECOVER => Ok(()),
                _ => Err(CoreError::JWTTypeError("Unexpected JWT type received")),
            },
            Err(err) => Err(CoreError::JWTValidationError(err.to_string())),
        }
    }

    pub fn check_password_recover_authorization(
        &self,
        tk: &str,
        usr_id: &str,
    ) -> Result<(), CoreError> {
        let user = self.user_service.find_by_id(usr_id)?;

        match decode::<Claims>(
            tk,
            &DecodingKey::from_secret(get_secret().as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => {
                if data.claims.email == user.email {
                    Ok(())
                } else {
                    Err(CoreError::InvalidCredentials(
                        "On password reset validation",
                    ))
                }
            }

            Err(err) => Err(CoreError::JWTValidationError(err.to_string())),
        }
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

fn generate_passwd_recover_exp() -> usize {
    match Utc::now().checked_add_signed(chrono::Duration::days(1)) {
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

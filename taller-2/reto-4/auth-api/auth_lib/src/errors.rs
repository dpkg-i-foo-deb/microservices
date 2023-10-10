use diesel::ConnectionError;
use password_auth::VerifyError;
use std::{error::Error, fmt::Display, fmt::Formatter};

#[derive(Debug)]
pub enum CoreError {
    DbConnectionError(diesel::ConnectionError),
    DbResultError(diesel::result::Error),
    UserNotFoundError(&'static str),
    InvalidCredentials(&'static str),
    JWTError(jsonwebtoken::errors::Error),
    JWTTypeError(&'static str),
    JWTValidationError(String),
}

impl Error for CoreError {}

impl From<ConnectionError> for CoreError {
    fn from(err: diesel::ConnectionError) -> Self {
        CoreError::DbConnectionError(err)
    }
}

impl From<diesel::result::Error> for CoreError {
    fn from(err: diesel::result::Error) -> Self {
        CoreError::DbResultError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for CoreError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        CoreError::JWTError(err)
    }
}

impl From<VerifyError> for CoreError {
    fn from(_: VerifyError) -> Self {
        CoreError::InvalidCredentials("Invalid password")
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CoreError::DbConnectionError(err) => {
                write!(f, "Database connection error {}", err)
            }
            CoreError::DbResultError(err) => {
                write!(f, "Error fetching a result from the database {}", err)
            }
            CoreError::UserNotFoundError(err) => {
                write!(f, "User not found {}", err)
            }
            CoreError::JWTError(err) => {
                write!(f, "{}", err)
            }
            CoreError::InvalidCredentials(err) => {
                write!(f, "Invalid credentials specified {}", err)
            }
            CoreError::JWTTypeError(err) => {
                write!(f, "The requested token type does not exist {}", err)
            }
            CoreError::JWTValidationError(err) => {
                write!(f, "Could not validate the token {err}")
            }
        }
    }
}

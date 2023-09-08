use std::{error::Error, fmt::Display, fmt::Formatter};

use diesel::ConnectionError;
#[derive(Debug)]
pub enum CoreError<'a> {
    DbConnectionError(diesel::ConnectionError),
    DbResultError(diesel::result::Error),
    UserNotFoundError(&'a str),
    JWTError(jsonwebtoken::errors::Error),
}

impl Error for CoreError<'_> {}

impl From<ConnectionError> for CoreError<'_> {
    fn from(err: diesel::ConnectionError) -> Self {
        CoreError::DbConnectionError(err)
    }
}

impl From<diesel::result::Error> for CoreError<'_> {
    fn from(err: diesel::result::Error) -> Self {
        CoreError::DbResultError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for CoreError<'_> {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        CoreError::JWTError(err)
    }
}

impl Display for CoreError<'_> {
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
        }
    }
}

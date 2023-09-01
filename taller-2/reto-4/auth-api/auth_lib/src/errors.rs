use std::{error::Error, fmt::Display, fmt::Formatter};

use diesel::ConnectionError;
#[derive(Debug)]
pub enum CoreError {
    DbConnectionError(diesel::ConnectionError),
}

impl Error for CoreError {}

impl From<ConnectionError> for CoreError {
    fn from(err: diesel::ConnectionError) -> Self {
        CoreError::DbConnectionError(err)
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CoreError::DbConnectionError(err) => {
                write!(f, "Database connection error {}", err)
            }
        }
    }
}

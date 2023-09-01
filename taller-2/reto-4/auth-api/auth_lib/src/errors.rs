use std::{error::Error, fmt::Display, fmt::Formatter};

use diesel::ConnectionError;
#[derive(Debug)]
pub enum CoreError {
    DbConnectionError(diesel::ConnectionError),
    DbResultError(diesel::result::Error),
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

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CoreError::DbConnectionError(err) => {
                write!(f, "Database connection error {}", err)
            }
            CoreError::DbResultError(err) => {
                write!(f, "Error fetching a result from the database {}", err)
            }
        }
    }
}

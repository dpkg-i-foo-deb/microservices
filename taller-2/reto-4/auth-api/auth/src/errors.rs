use std::{error::Error, fmt::Display};

use auth_lib::errors::CoreError;
use rocket::serde::json::Json;

use crate::views::error::ErrorPayload;
#[derive(Debug, Responder)]
pub enum ApiError {
    #[response(status = 400)]
    BadRequest(Json<ErrorPayload>),
    #[response(status = 401)]
    Unauthorized(Json<ErrorPayload>),
    #[response(status = 404)]
    EntityNotFound(Json<ErrorPayload>),
    #[response(status = 500)]
    InternalError(Json<ErrorPayload>),
}

impl Error for ApiError {}

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::DbConnectionError(_)
            | CoreError::DbResultError(_)
            | CoreError::JWTError(_) => ApiError::InternalError(ErrorPayload::internal_err()),
            CoreError::UserNotFoundError(_) => {
                ApiError::EntityNotFound(ErrorPayload::entity_not_found())
            }
            CoreError::InvalidCredentials(_) => {
                ApiError::Unauthorized(ErrorPayload::unauthorized())
            }
            CoreError::JWTTypeError(_) => ApiError::BadRequest(ErrorPayload::bad_request()),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error")
    }
}

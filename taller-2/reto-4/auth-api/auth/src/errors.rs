use std::{error::Error, fmt::Display};

use auth_lib::errors::CoreError;
use rocket::serde::json::Json;

use crate::views::error::ErrorPayload;

#[derive(Debug, Responder)]
pub enum ApiError {
    #[response(status = 500)]
    InternalError(Json<ErrorPayload>),
}

impl Error for ApiError {}

impl From<CoreError> for ApiError {
    fn from(_: CoreError) -> Self {
        ApiError::InternalError(ErrorPayload::internal_err())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error")
    }
}

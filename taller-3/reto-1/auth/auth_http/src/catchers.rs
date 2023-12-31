use rocket::catch;
use rocket::serde::json::Json;

use crate::views::error::ErrorPayload;

#[catch(500)]
pub fn internal_err() -> Json<ErrorPayload> {
    ErrorPayload::internal_err()
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorPayload> {
    ErrorPayload::unauthorized()
}

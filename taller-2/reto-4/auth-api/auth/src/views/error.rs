use rocket::serde::json::Json;
use serde::Serialize;

static INTERNAL_ERR: &str = "internal_error";
static INTERNAL_ERR_DESC: &str = "Something went wrong server-side";

#[derive(Serialize, Debug)]
pub struct ErrorPayload {
    error_id: &'static str,
    error_description: &'static str,
}

impl ErrorPayload {
    pub fn internal_err() -> Json<ErrorPayload> {
        let err = ErrorPayload {
            error_id: INTERNAL_ERR,
            error_description: INTERNAL_ERR_DESC,
        };

        Json(err)
    }
}

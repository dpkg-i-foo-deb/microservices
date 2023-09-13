use rocket::serde::json::Json;
use serde::Serialize;

static INTERNAL_ERR: &str = "internal_error";
static INTERNAL_ERR_DESC: &str = "Something went wrong server-side";

static ENTITY_NOT_FOUND: &str = "entity_not_found";
static ENTITY_NOT_FOUND_DESC: &str = "The requested entity was not found";

static UNAUTHORIZED: &str = "access_denied";
static UNAUTHORIZED_DESC: &str = "Invalid credentials provided";

static BAD_REQUEST: &str = "bad_request";
static BAD_REQUEST_DESC: &str = "There's a problem with the request data";

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

    pub fn entity_not_found() -> Json<ErrorPayload> {
        let err = ErrorPayload {
            error_id: ENTITY_NOT_FOUND,
            error_description: ENTITY_NOT_FOUND_DESC,
        };

        Json(err)
    }

    pub fn unauthorized() -> Json<ErrorPayload> {
        let err = ErrorPayload {
            error_id: UNAUTHORIZED,
            error_description: UNAUTHORIZED_DESC,
        };

        Json(err)
    }

    pub fn bad_request() -> Json<ErrorPayload> {
        let err = ErrorPayload {
            error_id: BAD_REQUEST,
            error_description: BAD_REQUEST_DESC,
        };

        Json(err)
    }
}

use rocket::{serde::json::Json, State};

use crate::{
    errors::ApiError,
    responders::ApiResponse,
    state::AppState,
    views::{credentials::CredentialsPayload, jwt::AuthTk},
};

#[post("/login", data = "<credentials>")]
pub fn login(
    credentials: Json<CredentialsPayload<'_>>,
    state: &State<AppState>,
) -> Result<ApiResponse<AuthTk>, ApiError> {
    state
        .login_service()
        .login(credentials.email(), credentials.password())?;

    let tk = state
        .jwt_service()
        .generate_access_tk("uwu?", credentials.email())?;

    Ok(ApiResponse::Succcess(Json(AuthTk::new(&tk))))
}

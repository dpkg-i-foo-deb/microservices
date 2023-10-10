use rocket::{post, serde::json::Json, State};

use crate::{
    errors::ApiError,
    responders::ApiResponse,
    state::AppState,
    views::{credentials::CredentialsPayload, jwt::Tk, login::LoginData},
};

#[post("/login", data = "<credentials>")]
pub fn login(
    credentials: Json<CredentialsPayload>,
    state: &State<AppState>,
) -> Result<ApiResponse<LoginData>, ApiError> {
    let tk = state
        .login_service()
        .login(credentials.email(), credentials.password())?;

    let tk = Tk::new(&tk);

    let data = LoginData {
        message: "Authenticated",
    };

    Ok(ApiResponse::Authenticated(Json(data), tk))
}

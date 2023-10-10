use rocket::{serde::json::Json, State};

use crate::{
    errors::ApiError,
    responders::ApiResponse,
    state::AppState,
    views::jwt::{Tk, TokensPayload},
};

#[post("/tokens", data = "<data>")]
pub fn tokens(
    data: Json<TokensPayload>,
    state: &State<AppState>,
) -> Result<ApiResponse<Tk>, ApiError> {
    let tk = state
        .jwt_service()
        .generate_tk(data.token_type, data.user_email, "role")?;

    let tk = Tk::new(&tk);

    Ok(ApiResponse::Succcess(Json(tk)))
}

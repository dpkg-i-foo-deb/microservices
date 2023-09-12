use crate::{
    errors::ApiError,
    responders::ApiResponse,
    views::user::{CreatedUser, NewUserPayload},
};

use auth_lib::models::user::NewUser;
use rocket::{post, serde::json::Json, State};

use crate::state::AppState;

#[post("/users", data = "<new_user>")]
pub fn register_user(
    new_user: Json<NewUserPayload<'_>>,
    state: &State<AppState>,
) -> Result<ApiResponse<CreatedUser>, ApiError> {
    let new_user = NewUser {
        email: new_user.email,
        username: new_user.username,
        password: new_user.password,
    };

    let user = state.user_service().create_user(&new_user)?;

    let user = CreatedUser::from_domain(user);

    Ok(ApiResponse::Created(Json(user)))
}

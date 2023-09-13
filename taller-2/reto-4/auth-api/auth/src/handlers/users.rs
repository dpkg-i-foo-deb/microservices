use crate::{
    errors::ApiError,
    responders::ApiResponse,
    views::user::{CreatedUser, ModifiedUserPayload, NewUserPayload},
};

use auth_lib::models::user::{ModifiedUser, NewUser};
use rocket::{post, serde::json::Json, State};

use crate::state::AppState;

#[post("/users", data = "<new_user>")]
pub fn register_user(
    new_user: Json<NewUserPayload>,
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

#[patch("/users/<id>", data = "<updated_user>")]
pub fn modify_user(updated_user: Json<ModifiedUserPayload>, state: &State<AppState>, id: &str) {
    let updated_user = ModifiedUser {
        email: updated_user.email,
        username: updated_user.username,
        password: updated_user.password,
    };

    state.user_service().modify_user(&updated_user, id);
}

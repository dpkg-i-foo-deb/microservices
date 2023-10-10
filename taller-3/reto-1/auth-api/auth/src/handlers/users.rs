use crate::{
    errors::ApiError,
    middleware::passwd_reset::PasswdResetGuard,
    responders::ApiResponse,
    views::{
        error::ErrorPayload,
        user::{
            CreatedUser, ListedUser, ModifiedUser as MdUser, ModifiedUserPayload, NewUserPayload,
        },
    },
};

use auth_lib::models::user::{ModifiedUser, NewUser};
use paginate::Pages;
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
pub fn modify_user(
    updated_user: Json<ModifiedUserPayload>,
    state: &State<AppState>,
    id: &str,
    guard: PasswdResetGuard,
) -> Result<ApiResponse<MdUser>, ApiError> {
    let updated_user = ModifiedUser {
        email: updated_user.email.unwrap_or(""),
        username: updated_user.username.unwrap_or(""),
        password: updated_user.password.unwrap_or(""),
    };

    state
        .jwt_service()
        .check_password_recover_authorization(&guard.get_token(), id)?;

    let updated_user = state.user_service().modify_user(&updated_user, id)?;

    let updated_user = MdUser::from_domain(updated_user);

    Ok(ApiResponse::Modified(Json(updated_user)))
}

#[get("/users?<limit>&<page>")]
pub fn list_users(
    state: &State<AppState>,
    limit: usize,
    page: usize,
) -> Result<ApiResponse<Vec<ListedUser>>, ApiError> {
    let users = state.user_service().fetch_all()?;

    let mut users = ListedUser::from_domain_list(users);

    let num_items = users.len();

    let pages = Pages::new(num_items, limit);

    let page = pages.with_offset(page);

    let users: Vec<ListedUser> = users.drain(page.start..=page.end).collect();

    Ok(ApiResponse::Succcess(Json(users)))
}

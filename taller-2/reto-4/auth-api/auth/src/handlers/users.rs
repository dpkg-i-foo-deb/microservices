use auth_lib::models::user::{NewUser, User};
use rocket::{http::Status, post, serde::json::Json, State};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct NewUserPayload<'c> {
    email: &'c str,
    password: &'c str,
    username: &'c str,
}
#[post("/users", data = "<new_user>")]
pub fn register_user(new_user: Json<NewUserPayload<'_>>, state: &State<AppState>) -> Status {
    let new_user = NewUser {
        email: new_user.email,
        username: new_user.username,
        password: new_user.password,
    };

    match state.user_service().create_user(&new_user) {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

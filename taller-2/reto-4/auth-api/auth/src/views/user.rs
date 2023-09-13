use auth_lib::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewUserPayload<'c> {
    pub email: &'c str,
    pub password: &'c str,
    pub username: &'c str,
}

#[derive(Deserialize)]
pub struct ModifiedUserPayload<'m> {
    pub email: &'m str,
    pub password: &'m str,
    pub username: &'m str,
}

#[derive(Serialize)]
pub struct CreatedUser {
    email: String,
    password: String,
    username: String,
}

impl CreatedUser {
    pub fn from_domain(user: User) -> CreatedUser {
        CreatedUser {
            email: user.email,
            password: user.password,
            username: user.username,
        }
    }
}

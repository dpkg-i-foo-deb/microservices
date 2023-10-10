use auth_lib::services::{jwt::JWTService, user::UserService};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct PasswdResetGuard(String);

#[derive(Debug)]
pub enum PasswdResetError {
    MissingTk,
    IncorrectTk,
    ExpiredTk,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PasswdResetGuard {
    type Error = PasswdResetError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => {
                let user_service = UserService::new();
                let jwt_service = JWTService::new(user_service);

                match jwt_service.check_passwd_recover_tk(token) {
                    Ok(()) => Outcome::Success(PasswdResetGuard(token.to_string())),
                    Err(_) => {
                        Outcome::Failure((Status::Unauthorized, PasswdResetError::IncorrectTk))
                    }
                }
            }
            None => Outcome::Failure((Status::Unauthorized, PasswdResetError::MissingTk)),
        }
    }
}

impl PasswdResetGuard {
    pub fn get_token(self) -> String {
        self.0
    }
}

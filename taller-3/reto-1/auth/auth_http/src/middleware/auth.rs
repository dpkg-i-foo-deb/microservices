use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct AuthGuard;

#[derive(Debug)]
pub enum AuthError {
    MissingTk,
    ExpiredTk,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => Outcome::Success(AuthGuard),
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingTk)),
        }
    }
}

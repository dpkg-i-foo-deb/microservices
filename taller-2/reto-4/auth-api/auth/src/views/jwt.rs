use serde::Serialize;

#[derive(Serialize)]
pub struct AuthTk {
    token: String,
}

impl AuthTk {
    pub fn new(token: &str) -> AuthTk {
        AuthTk {
            token: token.to_owned(),
        }
    }
}

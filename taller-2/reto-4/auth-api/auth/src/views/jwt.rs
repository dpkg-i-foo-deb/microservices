use rocket::http::Header;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Tk {
    token: String,
}

impl Tk {
    pub fn new(token: &str) -> Tk {
        Tk {
            token: token.to_owned(),
        }
    }
}

impl Into<Header<'_>> for Tk {
    fn into(self) -> Header<'static> {
        Header::new("AUTHORIZATION", self.token)
    }
}

#[derive(Deserialize)]
pub struct TokensPayload<'t> {
    pub token_type: &'t str,
    pub user_email: &'t str,
}

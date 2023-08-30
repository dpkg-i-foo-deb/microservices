use rocket::{post, serde::json::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Credentials>) {}

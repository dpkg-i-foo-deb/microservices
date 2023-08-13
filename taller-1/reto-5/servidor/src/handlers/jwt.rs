use rocket::serde::json::Json;

use crate::{auth, models::payload::Payload};

#[post("/jwt", data = "<payload>")]
pub fn get_jwt(payload: Json<Payload>) -> Json<Payload> {
    let data = auth::jwt::generate_jwt(&payload.data());

    Json(Payload::new(data))
}

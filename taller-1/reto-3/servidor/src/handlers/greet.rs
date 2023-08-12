use rocket::serde::json::Json;

use crate::{models::payload::Payload, services};

#[post("/greet",data="<payload>")]
pub fn greet(payload:Json<Payload>) -> Json<Payload>{
    let response = Payload::new(services::greet::build_greet(payload.data()));

    Json(response)
    
}
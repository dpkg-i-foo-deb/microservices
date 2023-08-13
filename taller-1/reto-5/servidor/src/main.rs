mod auth;
mod handlers;
mod models;

#[macro_use]
extern crate rocket;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handlers::jwt::get_jwt])
}

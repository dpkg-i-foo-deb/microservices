use crate::handlers::login::login;
use catchers::{internal_err, unauthorized};
use handlers::index::index;
use handlers::tokens::tokens;
use handlers::users::{list_users, modify_user, register_user};
use rocket::{routes, Build, Rocket};
use state::AppState;

mod catchers;
mod errors;
mod handlers;
mod middleware;
mod responders;
mod state;
mod views;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let rocket = build();

    match rocket.launch().await {
        Ok(_) => println!("Everything went well!!!"),
        Err(err) => eprintln!("{err}"),
    }
}

pub fn build() -> Rocket<Build> {
    let state = AppState::new();

    let rocket = rocket::build()
        .mount(
            "/",
            routes![index, register_user, modify_user, login, tokens, list_users],
        )
        .register("/", catchers![internal_err, unauthorized])
        .manage(state);

    rocket
}

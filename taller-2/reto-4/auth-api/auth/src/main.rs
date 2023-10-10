use crate::handlers::login::login;
use catchers::{internal_err, unauthorized};
use handlers::index::index;
use handlers::tokens::tokens;
use handlers::users::{list_users, modify_user, register_user};
use rocket::routes;
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
    match setup_app().await {
        Ok(_) => println!("Everything went well!!!"),
        Err(err) => eprintln!("{err}"),
    }
}

async fn setup_app() -> Result<(), rocket::Error> {
    let state = AppState::new();

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![index, register_user, modify_user, login, tokens, list_users],
        )
        .register("/", catchers![internal_err, unauthorized])
        .manage(state)
        .launch()
        .await?;

    Ok(())
}

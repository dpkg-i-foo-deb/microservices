use auth_lib::services::jwt::JWTService;
use auth_lib::services::login::LoginService;
use auth_lib::services::user::UserService;

use crate::handlers::login::login;
use catchers::internal_err;
use handlers::index::index;
use handlers::users::register_user;
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
    let user_service = UserService::new();
    let jwt_service = JWTService::new();
    let login_service = LoginService::new();

    let state = AppState::new(user_service, jwt_service, login_service);

    let _rocket = rocket::build()
        .mount("/", routes![index, register_user, login])
        .register("/", catchers![internal_err])
        .manage(state)
        .launch()
        .await?;

    Ok(())
}

use auth_lib::services::jwt::JWTService;
use auth_lib::services::user::UserService;

use handlers::users::register_user;
use rocket::routes;
use state::AppState;

mod handlers;
mod middleware;
mod state;

use handlers::index::index;

#[rocket::main]
async fn main() {
    match setup_app().await {
        Ok(_) => println!("Server stopped"),
        Err(err) => eprintln!("{err}"),
    }
}

async fn setup_app() -> Result<(), rocket::Error> {
    let user_service = UserService::new();
    let jwt_service = JWTService::new();

    let state = AppState::new(user_service, jwt_service);

    let _rocket = rocket::build()
        .mount("/", routes![index, register_user])
        .manage(state)
        .launch()
        .await?;

    Ok(())
}

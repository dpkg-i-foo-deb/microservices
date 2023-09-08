use std::env;
use std::net::SocketAddr;

use auth_lib::services::jwt::JWTService;
use auth_lib::services::user::UserService;

use axum::routing::get;
use axum::Router;
use state::AppState;

mod handlers;
mod middleware;
mod state;

#[tokio::main]
async fn main() {
    setup_app().await
}
async fn setup_app() {
    let user_service = UserService::new();
    let jwt_service = JWTService::new(get_secret());

    let state = AppState::new(user_service, jwt_service);

    let app = Router::new()
        .route("/", get(handlers::index::index))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_secret() -> String {
    match env::var("SECRET") {
        Ok(secret) => secret,
        Err(err) => panic!("Secret key is required {}", err),
    }
}

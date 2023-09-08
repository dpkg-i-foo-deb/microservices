use std::net::SocketAddr;

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

    let state = AppState::new(user_service);

    let app = Router::new()
        .route("/", get(handlers::index::index))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

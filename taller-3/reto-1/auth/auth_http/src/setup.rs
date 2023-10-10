use crate::handlers::login::login;
use crate::state::AppState;
use crate::{catchers, handlers};
use catchers::{internal_err, unauthorized};
use handlers::index::index;
use handlers::tokens::tokens;
use handlers::users::{list_users, modify_user, register_user};
use rocket::{routes, Build, Rocket};

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

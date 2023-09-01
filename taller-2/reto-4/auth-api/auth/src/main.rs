use auth_lib::models::user::NewUser;

use auth_lib::services::user::UserService;
use rocket::catch;
use rocket::serde::json::json;

mod routes;

#[catch(401)]
fn unauthorized() {
    json!({
        "error_id":"UNAUTHORIZED",
        "error_description":"Invalid credentials or session needed"
    });
}

fn main() {
    setup_services()
}

fn setup_services() {
    let user_service = UserService::new();

    let user = NewUser {
        email: "mateo.estradar@uqvirtual.edu.co",
        username: "dpkg",
        password: "michi",
    };

    user_service.create_user(&user).unwrap();
}

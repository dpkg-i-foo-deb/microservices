use auth_lib::models::user::NewUser;

use auth_lib::services::user::UserService;

mod routes;

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

    let user = user_service.create_user(&user).unwrap();

    let user = user_service.disable_user(&user).unwrap();
}

use auth_lib::database::DB;
use auth_lib::models::user::NewUser;
use auth_lib::models::user::User;
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
    let mut db = DB::new();

    setup_services(&mut db)
}

fn setup_services(db: &mut DB) {
    let mut user_service = UserService::new(&mut db.connection);

    let new_user = NewUser {
        email: "mateo.estradar@uqvirtual.edu.co".to_string(),
        username: "Mateo".to_string(),
        password: "michi".to_string(),
    };

    user_service.create_user(new_user);
}

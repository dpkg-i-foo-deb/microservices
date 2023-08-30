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
    println!("Hello, world!");
}

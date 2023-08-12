use crate::services;

#[get("/hello/<name>")]
pub fn hello(name: &str) -> String {
    services::hello::hello_user(name)
}

mod models;
mod handlers;
mod services;

#[macro_use] extern crate rocket;
#[launch]
fn rocket() ->_{
    rocket::build().mount("/", routes![handlers::greet::greet])
}

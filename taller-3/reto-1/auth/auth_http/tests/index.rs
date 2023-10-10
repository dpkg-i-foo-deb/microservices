use auth_http::setup;
use cucumber::{given, World};
use rocket::{http::Status, local::blocking::Client, uri};

#[derive(World, Debug, Default)]
struct Index {
    route: Option<String>,
}

#[given(expr = "I want to visit the {word} route")]
async fn visit_index(i: &mut Index, r: String) {
    i.route = Some(r);
}

#[tokio::main]
async fn main() {
    let rocket = setup::build();

    Index::run("tests/features/index.feature");

    let client = Client::tracked(rocket).expect("Rocket instance should be valid");

    let mut response = client
        .get(uri!(auth_http::handlers::index::index))
        .dispatch();

    assert_eq!(response.status(), Status::Ok)
}

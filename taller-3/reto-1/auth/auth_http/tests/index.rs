use auth_http::setup;
use cucumber::{given, then, when, World};
use rocket::{http::Status, local::asynchronous::Client, uri};

#[derive(World, Debug, Default)]
struct Index {
    client: Option<Client>,
    response_status: Status,
}

#[given(expr = "It is possible to connect to the system")]
async fn init_index(i: &mut Index) {
    let rocket = setup::build_testing();

    let client = Client::tracked(rocket)
        .await
        .expect("Rocket instance should be valid");

    i.client = Some(client);
}

#[when(expr = "I send a request to the index route")]
async fn request_index(i: &mut Index) {
    let response = i
        .client
        .as_ref()
        .unwrap()
        .get(uri!(auth_http::handlers::index::index))
        .dispatch()
        .await;

    i.response_status = response.status();
}

#[then(expr = "I get a successful response")]
async fn check_response(i: &mut Index) {
    assert_eq!(i.response_status, Status::Ok)
}

#[tokio::main]
async fn main() {
    Index::run("tests/features/index.feature").await;
}

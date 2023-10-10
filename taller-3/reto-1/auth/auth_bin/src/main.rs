use auth_http::setup::build;

#[rocket::main]
async fn main() {
    let rocket = build();

    match rocket.launch().await {
        Ok(_) => println!("Everything went well!!!"),
        Err(err) => eprintln!("{err}"),
    }
}

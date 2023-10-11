use auth_http::setup::build_release;

#[rocket::main]
async fn main() {
    let rocket = build_release();

    match rocket.launch().await {
        Ok(_) => println!("Everything went well!!!"),
        Err(err) => eprintln!("{err}"),
    }
}

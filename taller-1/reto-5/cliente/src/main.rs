use std::env;

use models::payload::Payload;
use reqwest::Response;

mod models;

#[tokio::main]
async fn main() {
    get_jwt().await
}

async fn get_jwt() {
    let url = setup_url();

    let payload = Payload::new(fetch_username());

    let json = payload.to_json();

    let result = match json {
        Some(json) => {
            let client = reqwest::Client::new();

            client.post(url).body(json).send().await
        }
        None => {
            panic!("Payload could not be constructed")
        }
    };

    match result {
        Ok(response) => {
            print_message(response).await;
        }
        Err(error) => {
            panic!("Failed to send the request {}", error)
        }
    }
}

async fn print_message(response: Response) {
    let data = response.json::<Payload>().await;

    match data {
        Ok(payload) => {
            println!("{}", payload.data())
        }
        Err(_) => {
            panic!("Payload could not be decoded")
        }
    }
}

fn fetch_username() -> String {
    let username = env::var("USERNAME");

    match username {
        Ok(usr) => usr,
        Err(err) => panic!("Couldn't find the username {err}"),
    }
}

fn setup_url() -> String {
    let env = env::var("URL");

    match env {
        Ok(url) => url,
        Err(_) => "http://localhost:8000/jwt/".to_string(),
    }
}

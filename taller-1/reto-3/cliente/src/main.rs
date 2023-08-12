mod models;

use models::payload::Payload;
use reqwest::Response;
use std::env;

#[tokio::main]
async fn main() {
    send_message().await;
}

fn fetch_username() -> String {
    let username = env::var("USERNAME");

    match username {
        Ok(usr) => usr,
        Err(_) => "USERNAME_NOT_DECLARED".to_string(),
    }
}

async fn send_message() {
    let payload = Payload::new(fetch_username());

    let json = payload.to_json();

    let result = match json {
        Some(json) => {
            let client = reqwest::Client::new();

            client.post("http://servidor/greet").body(json).send().await


        },
        None=>{panic!("Payload could not be constructed")}
    };

    match result {
        Ok(response)=>{
            print_message(response).await;
        },
        Err(_)=>{panic!("Failed to send the request")}
    }
}

async fn print_message(response:Response){
    let data = response.json::<Payload>().await;

    match data {
        Ok(payload)=>{
            println!("{}",payload.data())
        },
        Err(_)=>{panic!("Payload could not be decoded")}
    }
}

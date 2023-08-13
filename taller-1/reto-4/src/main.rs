use std::io;

use auth::jwt::generate_jwt;

mod auth;

fn main() {
    read_username()
}

fn read_username() {
    let mut input = String::new();

    println!("Enter username:");

    let stdin = io::stdin();

    match stdin.read_line(&mut input) {
        Ok(_) => get_jwt(&input.trim_end()),
        Err(err) => panic!("Error: {err}"),
    };
}

fn get_jwt(usr: &str) {
    let tk = generate_jwt(usr);

    println!("\n\nYour token is:\n\n{}\n\n", tk)
}

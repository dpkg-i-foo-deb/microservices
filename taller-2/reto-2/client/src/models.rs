use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sessions {
    id: String,
    start: String,
    end: String,
    user_id: String,
}

impl User {
    pub fn new(id: &str, email: &str, username: &str, password: &str) -> User {
        User {
            id: id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

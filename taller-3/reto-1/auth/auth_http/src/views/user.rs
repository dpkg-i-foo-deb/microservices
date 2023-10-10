use domain::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewUserPayload<'c> {
    pub email: &'c str,
    pub password: &'c str,
    pub username: &'c str,
}

#[derive(Deserialize)]
pub struct ModifiedUserPayload<'m> {
    pub email: Option<&'m str>,
    pub password: Option<&'m str>,
    pub username: Option<&'m str>,
}

#[derive(Serialize)]
pub struct CreatedUser {
    email: String,
    password: String,
    username: String,
}

impl CreatedUser {
    pub fn from_domain(user: User) -> CreatedUser {
        CreatedUser {
            email: user.email,
            password: user.password,
            username: user.username,
        }
    }
}

#[derive(Serialize)]
pub struct ModifiedUser {
    email: String,
    password: String,
    username: String,
}

impl ModifiedUser {
    pub fn from_domain(user: User) -> ModifiedUser {
        ModifiedUser {
            email: user.email,
            password: user.password,
            username: user.username,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ListedUser {
    email: String,
    username: String,
}

impl ListedUser {
    pub fn from_domain_list(users: Vec<User>) -> Vec<ListedUser> {
        let mut result: Vec<ListedUser> = vec![];

        for user in users.into_iter() {
            result.push(ListedUser {
                email: user.email,
                username: user.username,
            })
        }

        result
    }
}

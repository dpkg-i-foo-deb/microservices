use serde::Deserialize;

#[derive(Deserialize)]
pub struct CredentialsPayload<'c> {
    email: &'c str,
    password: &'c str,
}

impl CredentialsPayload<'_> {
    pub fn email(&self) -> &str {
        self.email
    }

    pub fn password(&self) -> &str {
        self.password
    }
}

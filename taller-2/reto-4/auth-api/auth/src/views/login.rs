use serde::Serialize;

#[derive(Serialize)]
pub struct LoginData {
    pub message: &'static str,
}

use rocket::{response::Responder, serde::json::Json};

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 201)]
    Created(Json<T>),
}

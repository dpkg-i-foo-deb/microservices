use rocket::{response::Responder, serde::json::Json};

use crate::views::jwt::Tk;

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200)]
    Authenticated(Json<T>, Tk),
    #[response(status = 201)]
    Created(Json<T>),
    #[response(status = 200)]
    Succcess(Json<T>),
}

use auth_lib::services::{jwt::JWTService, user::UserService};

#[derive(Clone)]
pub struct AppState {
    user_service: UserService,
    jwt_service: JWTService,
}

impl AppState {
    pub fn new(user_service: UserService, jwt_service: JWTService) -> AppState {
        AppState {
            user_service,
            jwt_service,
        }
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }
}

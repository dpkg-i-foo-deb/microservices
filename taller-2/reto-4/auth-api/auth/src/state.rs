use auth_lib::services::{jwt::JWTService, login::LoginService, user::UserService};

pub struct AppState {
    user_service: UserService,
    jwt_service: JWTService,
    login_service: LoginService,
}

impl AppState {
    pub fn new(
        user_service: UserService,
        jwt_service: JWTService,
        login_service: LoginService,
    ) -> AppState {
        AppState {
            user_service,
            jwt_service,
            login_service,
        }
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }

    pub fn login_service(&self) -> &LoginService {
        &self.login_service
    }

    pub fn jwt_service(&self) -> &JWTService {
        &self.jwt_service
    }
}

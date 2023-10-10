use domain::services::{jwt::JWTService, login::LoginService, user::UserService};

pub struct AppState {
    user_service: UserService,
    jwt_service: JWTService,
    login_service: LoginService,
}

impl AppState {
    pub fn new() -> AppState {
        let user_service = UserService::new();
        let jwt_service = JWTService::new(UserService::new());
        let login_service = LoginService::new(JWTService::new(UserService::new()));

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

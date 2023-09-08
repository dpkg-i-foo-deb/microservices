use auth_lib::services::user::UserService;

#[derive(Clone)]
pub struct AppState {
    user_service: UserService,
}

impl AppState {
    pub fn new(user_service: UserService) -> AppState {
        AppState { user_service }
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }
}

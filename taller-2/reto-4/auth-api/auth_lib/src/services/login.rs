use crate::database::DB;

pub struct LoginService {
    db: DB,
}

impl LoginService {
    pub fn new() -> LoginService {
        LoginService { db: (DB::new()) }
    }

    pub fn login(email: &str, password: &str) {}
}

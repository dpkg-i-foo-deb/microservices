use crate::models::user::{NewUser, User};
use diesel::PgConnection;
use password_auth::generate_hash;
use uuid::Uuid;

pub struct UserService<'s> {
    db: &'s mut PgConnection,
}

impl UserService<'_> {
    pub fn new(db: &mut PgConnection) -> UserService {
        UserService { db }
    }

    pub fn create_user(&mut self, new_user: NewUser) -> Option<User> {
        let id = generate_uuid();

        let hash = generate_hash(new_user.password);

        let user = User {
            id,
            email: new_user.email,
            password: hash,
            username: new_user.username,
            status: "CREATED".to_string(),
        };

        user.create(&mut self.db)
    }
}

fn generate_uuid() -> String {
    let id = Uuid::new_v4();

    id.to_string()
}

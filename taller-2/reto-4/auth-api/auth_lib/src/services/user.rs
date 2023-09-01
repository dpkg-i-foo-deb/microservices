use crate::database::DB;
use crate::errors::CoreError;
use crate::models::user::{NewUser, User};
use crate::schema::users;

use diesel::RunQueryDsl;
use password_auth::generate_hash;
use uuid::Uuid;

pub struct UserService {
    db: DB,
}

impl UserService {
    pub fn new() -> UserService {
        let db = DB::new();

        UserService { db }
    }

    pub fn create_user(&self, new_user: &NewUser) -> Result<User, CoreError> {
        let mut conn = self.db.establish_connection()?;

        let id = generate_uuid();

        let hash = generate_hash(&new_user.password);

        let user = User {
            id,
            email: new_user.email.to_owned(),
            password: hash,
            username: new_user.username.to_owned(),
            status: "CREATED".to_string(),
        };

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(&mut conn)?;

        Ok(user)
    }
}

fn generate_uuid() -> String {
    let id = Uuid::new_v4();

    id.to_string()
}

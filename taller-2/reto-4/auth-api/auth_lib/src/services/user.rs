use crate::database::DB;
use crate::errors::CoreError;
use crate::models::user::{NewUser, User};
use crate::schema::users;

use diesel::{ExpressionMethods, RunQueryDsl};
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

    pub fn modify_user(&self, modified_user: &User) -> Result<User, CoreError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.establish_connection()?;

        let modified_user = diesel::update(users)
            .set((
                email.eq(&modified_user.email),
                username.eq(&modified_user.username),
                status.eq("MODIFIED"),
            ))
            .filter(id.eq(&modified_user.id))
            .get_result::<User>(&mut conn)?;

        Ok(modified_user)
    }

    pub fn disable_user(&self, user: &User) -> Result<User, CoreError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.establish_connection()?;

        let user = diesel::update(users)
            .set(status.eq("DISABLED"))
            .filter(id.eq(&user.id))
            .get_result::<User>(&mut conn)?;

        Ok(user)
    }

    pub fn update_password(&self, user_id: &str, passwd: &str) -> Result<bool, CoreError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.establish_connection()?;

        let hash = generate_hash(passwd);

        diesel::update(users)
            .set(password.eq(hash))
            .filter(id.eq(user_id))
            .execute(&mut conn)?;

        Ok(true)
    }
}

fn generate_uuid() -> String {
    let id = Uuid::new_v4();

    id.to_string()
}

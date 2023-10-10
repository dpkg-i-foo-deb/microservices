use crate::database::DB;
use crate::errors::CoreError;
use crate::models::user::{ModifiedUser, NewUser, User};
use crate::schema::users::dsl::users;
use diesel::prelude::*;

use diesel::{ExpressionMethods, RunQueryDsl};
use password_auth::generate_hash;
use uuid::Uuid;

pub struct UserService {
    db: DB,
}

impl UserService {
    pub fn new() -> UserService {
        UserService { db: DB::new() }
    }

    pub fn fetch_all(&self) -> Result<Vec<User>, CoreError> {
        use crate::schema::users;

        let mut conn = self.db.establish_connection()?;

        let result = users::table.load::<User>(&mut conn)?;

        Ok(result)
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

        let user = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&mut conn)?;

        Ok(user)
    }

    pub fn modify_user(
        &self,
        modified_user: &ModifiedUser,
        usr_id: &str,
    ) -> Result<User, CoreError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.establish_connection()?;

        match modified_user.email {
            "" => {}
            _ => {
                diesel::update(users)
                    .set((email.eq(&modified_user.email), status.eq("MODIFIED")))
                    .filter(id.eq(usr_id))
                    .get_result::<User>(&mut conn)?;
            }
        }

        match modified_user.username {
            "" => {}
            _ => {
                diesel::update(users)
                    .set((username.eq(&modified_user.username), status.eq("MODIFIED")))
                    .filter(id.eq(usr_id))
                    .get_result::<User>(&mut conn)?;
            }
        }

        match modified_user.password {
            "" => {}
            _ => self.update_password(usr_id, &modified_user.password)?,
        };

        let modified_user = users
            .filter(id.eq(usr_id))
            .select(User::as_select())
            .first(&mut conn)?;

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

    fn update_password(&self, user_id: &str, passwd: &str) -> Result<(), CoreError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.db.establish_connection()?;

        let hash = generate_hash(passwd);

        diesel::update(users)
            .set(password.eq(hash))
            .filter(id.eq(user_id))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn find_by_email(&self, user_email: &str) -> Result<User, CoreError> {
        use crate::schema::users::dsl::email;

        let mut conn = self.db.establish_connection()?;

        let user = users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;

        match user {
            Some(user) => Ok(user),
            None => Err(CoreError::UserNotFoundError("User not found")),
        }
    }

    pub fn find_by_id(&self, usr_id: &str) -> Result<User, CoreError> {
        use crate::schema::users::dsl::id;

        let mut conn = self.db.establish_connection()?;

        let user = users
            .filter(id.eq(usr_id))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;

        match user {
            Some(user) => Ok(user),
            None => Err(CoreError::UserNotFoundError("User not found")),
        }
    }
}

fn generate_uuid() -> String {
    let id = Uuid::new_v4();

    id.to_string()
}

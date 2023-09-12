use crate::{database::DB, errors::CoreError, models::user::User, schema::users::dsl::users};
use diesel::prelude::*;
use password_auth::verify_password;

pub struct LoginService {
    db: DB,
}

impl LoginService {
    pub fn new() -> LoginService {
        LoginService { db: (DB::new()) }
    }

    pub fn login(&self, user_email: &str, user_passwd: &str) -> Result<(), CoreError> {
        use crate::schema::users::dsl::email;

        let mut conn = self.db.establish_connection()?;

        let result = users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()?;

        match result {
            Some(user) => {
                verify_password(user_passwd, &user.password)?;

                Ok(())
            }
            None => Err(CoreError::UserNotFoundError("On login attempt")),
        }
    }
}

use crate::{database::DB, errors::CoreError, models::user::User, schema::users::dsl::users};
use diesel::prelude::*;

pub struct LoginService {
    db: DB,
}

impl LoginService {
    pub fn new() -> LoginService {
        LoginService { db: (DB::new()) }
    }

    pub fn login(&self, user_email: &str, user_passwd: &str) -> Result<(), CoreError> {
        use crate::schema::users::dsl::{email, password};

        let mut conn = self.db.establish_connection()?;

        let result = users
            .filter(email.eq(user_email).and(password.eq(user_passwd)))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()?;

        match result {
            Some(_) => Ok(()),
            None => Err(CoreError::InvalidCredentials("On login attempt")),
        }
    }
}

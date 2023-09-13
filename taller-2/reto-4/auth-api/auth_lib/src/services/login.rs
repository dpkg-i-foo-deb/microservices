use crate::services::jwt::JWTService;
use crate::{database::DB, errors::CoreError, models::user::User, schema::users::dsl::users};
use diesel::prelude::*;
use password_auth::verify_password;

pub struct LoginService {
    jwt_service: JWTService,
    db: DB,
}

impl LoginService {
    pub fn new(jwt_service: JWTService) -> LoginService {
        LoginService {
            db: (DB::new()),
            jwt_service,
        }
    }

    //TODO mirar si es posible retornar los dos tipos de token de una vez
    pub fn login(&self, user_email: &str, user_passwd: &str) -> Result<String, CoreError> {
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

                let tk = self.jwt_service.generate_access_tk("ADMIN", user_email)?;

                Ok(tk)
            }
            None => Err(CoreError::UserNotFoundError("On login attempt")),
        }
    }
}

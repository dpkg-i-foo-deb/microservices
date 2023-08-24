use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

impl User {
    pub fn create(&self, conn: &mut PgConnection) -> Option<User> {
        let result = diesel::insert_into(schema::users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn);

        match result {
            Ok(user) => Some(user),
            Err(err) => {
                eprintln!("{err}");

                None
            }
        }
    }

    pub fn list_all(conn: &mut PgConnection) -> Vec<User> {
        match schema::users::table.load::<User>(conn) {
            Ok(users) => users,
            Err(err) => {
                eprintln!("{err}");

                vec![]
            }
        }
    }
}

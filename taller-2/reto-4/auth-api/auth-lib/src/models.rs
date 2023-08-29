use crate::schema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub status: String,
}

impl User {
    pub fn create(&self, conn: &mut PgConnection) -> Option<User> {
        let id = generate_uuid();

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

fn generate_uuid() -> String {
    let id = Uuid::new_v4();

    id.to_string()
}

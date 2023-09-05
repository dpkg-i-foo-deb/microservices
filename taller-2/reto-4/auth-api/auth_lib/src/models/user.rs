use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub status: String,
}

pub struct NewUser<'u> {
    pub email: &'u str,
    pub username: &'u str,
    pub password: &'u str,
}

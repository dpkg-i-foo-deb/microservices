use crate::errors::CoreError;
use std::env::{self};

use diesel::{Connection, PgConnection};

#[derive(Copy, Clone)]
pub struct DB {}

impl DB {
    pub fn new() -> DB {
        DB {}
    }

    pub fn establish_connection(self) -> Result<PgConnection, CoreError<'static>> {
        let db_url = Self::fetch_db_url();

        let conn = PgConnection::establish(&db_url)?;

        Ok(conn)
    }

    fn fetch_db_url() -> String {
        match env::var("CONNECTION_STRING") {
            Ok(conn) => conn,
            Err(err) => panic!("Database string must be set {}", err),
        }
    }
}

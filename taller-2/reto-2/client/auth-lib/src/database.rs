use std::env;

use diesel::{Connection, PgConnection};

pub struct DB {
    pub connection: PgConnection,
}

impl DB {
    pub fn new() -> Option<DB> {
        match Self::establish_connection() {
            Ok(connection) => Some(DB { connection }),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }

    fn establish_connection() -> Result<PgConnection, diesel::ConnectionError> {
        let result = env::var("CONNECTION_STRING");

        let connection_string = match result {
            Ok(conn) => conn,
            Err(err) => panic!("Could not fetch the connection string {err}"),
        };

        PgConnection::establish(&connection_string)
    }
}

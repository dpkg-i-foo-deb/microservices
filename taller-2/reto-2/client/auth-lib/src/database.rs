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
        PgConnection::establish("postgres://dpkg:dpkg@localhost/auth")
    }
}

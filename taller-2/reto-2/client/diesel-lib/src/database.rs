use diesel::{Connection, PgConnection};

pub fn establish_connection() -> PgConnection {
    let result = PgConnection::establish("postgres://username:password@localhost/diesel_demo");

    match result {
        Ok(conn) => conn,
        Err(err) => {
            panic!("{err}")
        }
    }
}

use std::env;
use std::thread::sleep;
use std::time;

use diesel::{Connection, PgConnection};

pub struct DB {
    pub connection: PgConnection,
}

impl DB {
    pub fn new() -> DB {
        DB {
            connection: Self::establish_connection(),
        }
    }

    fn establish_connection() -> PgConnection {
        let result = env::var("CONNECTION_STRING");

        let connection_string = match result {
            Ok(conn) => conn,
            Err(err) => panic!("Could not fetch the connection string {err}"),
        };

        let mut counter: u8 = 0;

        let result = loop {
            match PgConnection::establish(&connection_string) {
                Ok(conn) => break conn,
                Err(err) => match counter {
                    0..=5 => {
                        eprintln!("Could not connect to the database {err}");
                        eprintln!("Retrying...");
                        sleep(time::Duration::from_secs(5));
                        counter += 1;
                    }
                    _ => {
                        panic!("Giving up to connect... {err}")
                    }
                },
            }
        };

        println!("Connected to the database!");

        result
    }
}

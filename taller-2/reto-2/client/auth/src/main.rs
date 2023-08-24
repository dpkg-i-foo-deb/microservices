use auth_lib::database::DB;
use auth_lib::models::User;

fn main() {
    run()
}

fn run() {
    let mut db = connect();
    create_user(&mut db);
    list_users(&mut db);
}

fn connect() -> DB {
    match DB::new() {
        Some(db) => db,
        None => panic!("Failed to connect to the database"),
    }
}

fn create_user(db: &mut DB) {
    let user = auth_lib::models::User {
        id: "01".to_string(),
        email: "someone@example.com".to_string(),
        password: "someone".to_string(),
        username: "some_user".to_string(),
    };

    user.create(&mut db.connection);
}

fn list_users(db: &mut DB) {
    let users = User::list_all(&mut db.connection);

    println!("{:?}", users)
}

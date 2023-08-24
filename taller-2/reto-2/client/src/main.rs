use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

mod models;

#[tokio::main]
async fn main() {
    println!("Connecting to the database...");

    match db_connect().await {
        Ok(_) => {
            println!("Connected!")
        }
        Err(err) => {
            panic!("{err}")
        }
    }
}

async fn db_connect() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("auth").use_db("auth").await?;

    let user = models::User::new("1", "someone@example.com", "someone", "someone");

    db.create("user").content(user).await?;

    Ok(())
}

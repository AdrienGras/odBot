use anyhow::Result;
use log::debug;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root};

pub async fn get_connection() -> Result<Surreal<Client>> {
    debug!("Connecting to surreal server...");
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    debug!("surreal server connected !");

    debug!("Authenticating against database...");
    db.signin(Root {
        username: "root",
        password: "toor",
    })
    .await?;
    debug!("Authenticated !");

    debug!("Setting up database client...");
    db.use_ns("test").use_db("test").await?;
    debug!("Setup complete !");

    Ok(db)
}
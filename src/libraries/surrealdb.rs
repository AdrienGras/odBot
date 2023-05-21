use std::{env, format};

use anyhow::Result;
use log::debug;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::core::constants;

pub async fn get_connection() -> Result<Surreal<Client>> {
    let host: String = env::var(constants::SURREAL_DB_HOST)?.parse()?;
    let port: i32 = env::var(constants::SURREAL_DB_PORT)?.parse()?;
    let user: String = env::var(constants::SURREAL_DB_USER)?.parse()?;
    let pass: String = env::var(constants::SURREAL_DB_PASS)?.parse()?;
    let db_name: String = env::var(constants::SURREAL_DB_NAME)?.parse()?;

    debug!("Connecting to surreal server...");
    debug!(" - host: {}", &host);
    debug!(" - port: {}", &port);
    debug!(" - user: {}", &user);
    debug!(" - pass: {}", &pass);
    debug!(" - db_name: {}", &db_name);

    let db = Surreal::new::<Ws>(format!("{}:{}", &host, &port)).await?;
    debug!("surreal server connected !");

    debug!("Authenticating against database...");
    db.signin(Root {
        username: &user,
        password: &pass,
    })
    .await?;
    debug!("Authenticated !");

    debug!("Setting up database client...");
    db.use_ns(&db_name).use_db(&db_name).await?;
    debug!("Setup complete !");

    Ok(db)
}

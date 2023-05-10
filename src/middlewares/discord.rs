use std::env;

use anyhow::Result;
use serenity::{prelude::GatewayIntents, Client};

use crate::middlewares::event_handler;

pub async fn create_client() -> Result<Client> {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let client = Client::builder(token, GatewayIntents::empty())
        .event_handler(event_handler::Handler)
        .await?;

    Ok(client)
}

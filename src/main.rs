mod commands;
mod middlewares;
mod utils;

use anyhow::Result;
use middlewares::discord;
use utils::{environment, logger};

#[tokio::main]
async fn main() -> Result<()> {
    // loading .env.local content into system environment
    // let it crash since it is a critical error
    environment::load()?;

    // load logger
    logger::load();

    // create client
    // let it crash since it is a critical error
    let mut client = discord::create_client().await?;

    // Finally, start a single shard, and start listening to events.
    // let it crash since it is a critical error
    client.start_autosharded().await?;

    // ok
    Ok(())
}

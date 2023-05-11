mod core;
mod libraries;
mod utils;
mod handlers;
mod controllers;
mod middlewares;

use crate::core::launcher::{self, Cli, Command};
use crate::core::application_context::ApplicationContext;
use anyhow::Result;
use clap::Parser;
use handlers::bot::Bot;
use libraries::discord;
use log::debug;
use utils::{environment, logger};

#[tokio::main]
async fn main() -> Result<()> {
    // loading .env.local content into system environment
    // let it crash since it is a critical error
    environment::load()?;

    // load logger
    logger::load();

    debug!("Starting surrealDB interface...");

    debug!("SurrealDB interface started !");

    debug!("Generating application context...");
    let app = ApplicationContext::new();
    debug!("Application context generated !");

    // parsing CLI class to dispatch action
    let cli = Cli::parse();

    // either trigger the launch of the web API service, or execute a given command.
    match &cli.command {
        // launch discord bot daemon.
        Command::Launch => {
            let discord_client = discord::create_bot_client(Bot::new(app)).await?;

            launcher::launch_server(discord_client).await?
        },
        // executing given command
        Command::Console { sub_command, args } => {
            let discord_client = discord::create_console_client().await?;

            launcher::launch_command(discord_client, app, sub_command, args).await?
        }
    }

    // ok
    Ok(())
}

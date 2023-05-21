mod commands;
mod controllers;
mod core;
mod data;
mod handlers;
mod libraries;
mod middlewares;
mod models;
mod utils;
mod renderers;

use crate::core::application_context::ApplicationContext;
use crate::core::launcher::{self, Cli, Command};
use anyhow::Result;
use clap::Parser;
use handlers::bot::Bot;
use libraries::{discord, surrealdb};
use log::{debug, error};
use utils::{environment, logger};

#[tokio::main]
async fn main() -> Result<()> {
    // loading .env.local content into system environment
    // let it crash since it is a critical error
    environment::load()?;

    // load logger
    logger::load();
    debug!("Environment and logger loaded !");

    debug!("Starting surrealDB interface...");
    let db = surrealdb::get_connection().await?;
    debug!("SurrealDB interface started !");

    debug!("Generating application context...");
    let app = ApplicationContext::new(db);
    debug!("Application context generated !");

    // parsing CLI class to dispatch action
    debug!("Parsing CLI command & args...");
    let cli = Cli::parse();
    debug!("CLI parsed !");

    // either trigger the launch of the web API service, or execute a given command.
    match &cli.command {
        // launch discord bot daemon.
        Command::Launch => {
            debug!("Initializing discord client...");
            let discord_client = discord::create_bot_client(Bot::new(app)).await?;
            debug!("Discord client initialized !");

            debug!("Starting discord bot daemon...");
            if let Err(error) = launcher::launch_server(discord_client).await {
                error!("Discord bot daemon crashed: {:?}", error);
            }
            debug!("Shutting down discord bot daemon...");
        }
        // executing given command
        Command::Console { sub_command, args } => {
            debug!("Initializing discord client...");
            let discord_client = discord::create_console_client().await?;
            debug!("Discord client initialized !");

            debug!("Starting command interface...");
            launcher::launch_command(discord_client, app, sub_command, args).await?;
            debug!("Shutting down command interface...");
        }
    }

    debug!("Shutting down...");
    Ok(())
}

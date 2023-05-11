mod core;
mod libraries;
mod utils;

use crate::core::application::ApplicationContext;

use crate::core::launcher::{self, Cli, Command};
use anyhow::Result;
use clap::Parser;
use utils::{environment, logger};

#[tokio::main]
async fn main() -> Result<()> {
    // loading .env.local content into system environment
    // let it crash since it is a critical error
    environment::load()?;

    // load logger
    logger::load();

    // initializing application context
    // this will :
    // - launch database
    // - create discord client
    let app_context = ApplicationContext::initialize();

    // parsing CLI class to dispatch action
    let cli = Cli::parse();

    // either trigger the launch of the web API service, or execute a given command.
    match &cli.command {
        // launch discord bot daemon.
        Command::Launch => launcher::launch_server(&app_context).await?,
        // executing given command
        Command::Console { sub_command, args } => {
            launcher::launch_command(&app_context, sub_command, args).await?
        }
    }

    // ok
    Ok(())
}

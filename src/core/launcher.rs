use std::collections::HashMap;

use anyhow::{Result, bail};
use clap::{builder::ValueParser, Parser, Subcommand};
use log::debug;
use serenity::Client;

use crate::{libraries::discord, core::commands};

use super::application_context::ApplicationContext;

/// A structure representing the principal running option for this program.
///
/// As this program takes a mandatory first argument, the real functional description is in the Command struct.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// sub-command to actually run a part of the program.
    #[command(subcommand)]
    pub command: Command,
}

/// actual mode switching intelligence for the program.
///
/// The `launch` argument will launch the web API as a long-running program.
/// The `console` argument will launch the command interface, executing a one-time command.
#[derive(Subcommand)]
pub enum Command {
    /// launches the discord bot daemon.
    Launch,
    /// launches the command interface.
    Console {
        /// the current command name to launch
        sub_command: String,
        /// args for this command, with format "<key>=<value>;<flag>;..."
        #[arg(num_args(0..), value_parser = ValueParser::new(parse_subcommand_args))]
        args: HashMap<String, Option<String>>,
    },
}

pub async fn launch_server(discord_client: Client) -> Result<()> {
    debug!("Starting discord bot daemon...");

    discord::start_client(discord_client).await?;

    debug!("Shutting down discord bot daemon...");
    Ok(())
}

pub async fn launch_command(
    discord: Client,
    app: ApplicationContext,
    sub_command: &str,
    args: &HashMap<String, Option<String>>,
) -> Result<()> {
    debug!("Generating command registry...");
    let command_registry = commands::get_command_registry();
    debug!("Command registry context generated !");

    debug!("Searching registry for command...");
    let command_instance = command_registry.get(sub_command);

    if command_instance.is_none() {
        bail!("Cannot find command: {:?}", sub_command);
    }

    debug!("Command found, entering command...");
    Ok(command_instance.unwrap().run(discord, app, args).await?)
}

/// This function will parse the arg string into a map formatted as KEY => Option(VALUE).
///
/// The format of the args must be :
/// - `key=val` for key-value pairs
/// - `flag` for flags only
///
/// All separated by `;`
fn parse_subcommand_args(arg_str: &str) -> Result<HashMap<String, Option<String>>> {
    let mut args = HashMap::<String, Option<String>>::new();

    let arg_packs = arg_str.split(';').collect::<Vec<&str>>();

    for arg_pack in arg_packs.iter() {
        if arg_pack.contains('=') {
            let arg_body = arg_pack.split('=').collect::<Vec<&str>>();

            let arg_name = arg_body.first().unwrap().to_string();
            let arg_value = arg_body.get(1).unwrap().to_string();

            args.insert(arg_name, Some(arg_value));
        } else {
            args.insert(arg_pack.to_string(), None);
        }
    }

    Ok(args)
}

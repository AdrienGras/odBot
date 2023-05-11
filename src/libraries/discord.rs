use std::env;

use anyhow::{Result, bail};
use log::debug;
use serenity::{Client, prelude::{GatewayIntents, Context}, model::{prelude::interaction::{application_command::{ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue}, InteractionResponseType}, user::User}};
use serenity_ctrlc::{Ext, Disconnector};

use crate::handlers::bot::Bot;

pub async fn create_bot_client(bot: Bot) -> Result<Client> {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let client = Client::builder(token, GatewayIntents::empty())
        // add bot to event handler
        .event_handler(bot)
        .await?;

    Ok(client)
}

pub async fn create_console_client() -> Result<Client> {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let client = Client::builder(token, GatewayIntents::empty())
        .await?;

    Ok(client)
}

pub async fn start_client(client: Client) -> Result<()> {
    client.ctrlc_with(|dc| async {
        debug!("CTRL+C recieved, disconnecting daemon...");
        Disconnector::disconnect_some(dc).await;
    })?.start_autosharded().await?;

    Ok(())
}

pub async fn respond_with_message(command: &ApplicationCommandInteraction, ctx: &Context, content: String) -> Result<(), serenity::Error> {
    command
        .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(content))
        })
    .await
}

pub fn resolve_user_arg(arg: &CommandDataOption) -> Result<&User> {
    let arg_val = arg
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::User(user, _member) = arg_val {
        Ok(user)
    } else {
        bail!("Please provide a valid user");
    }
}

pub fn resolve_int_arg(arg: &CommandDataOption) -> Result<i64> {
    let arg_val = arg
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::Integer(val) = arg_val {
        Ok(*val)
    } else {
        bail!("Please provide a valid integer");
    }
}
use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::commands::command_trait::DiscordSlashCommand;
use crate::commands::ping::PingCommand;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!(
                "Received command interaction: {:?} ({:?}) by {:?}",
                command.data.name,
                command.data.options,
                command.user.tag()
            );

            let content = match command.data.name.as_str() {
                "ping" => PingCommand::run(&command.data.options, &command.user),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| PingCommand::register(command))
        })
        .await
        .unwrap();

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
                .iter()
                .map(|command| command.name.clone())
                .collect::<String>()
        );
    }
}

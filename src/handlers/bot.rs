use std::env;

use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error};
use serenity::{prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId, interaction::Interaction}};

use crate::{core::{application_context::ApplicationContext, controllers::{SlashCommandControllerTrait}, constants}, controllers::{ ping::PingController}};

pub struct Bot {
    application_context: ApplicationContext,
}

impl Bot {
    pub fn new(application_context: ApplicationContext) -> Self {
        Self { application_context }
    }
}

#[async_trait]
impl EventHandler for Bot {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!(
                "Received command interaction: {:?} ({:?}) by {:?}",
                command.data.name,
                command.data.options,
                command.user.tag()
            );

            let result = match command.data.name.as_str() {
                "ping" => PingController::run(&command, &ctx, &self.application_context).await,
                _ => Err(anyhow!("Not implemented !")),
            };

            if let Err(error) = result {
                debug!(
                    "Received command interaction: {:?} ({:?}) by {:?} -> {:?}",
                    command.data.name,
                    command.data.options,
                    command.user.tag(),
                    error
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        debug!("Bot {} is connected!", ready.user.name);

        debug!("Adding guild slash commands...");
        let unparsed_guild_id = env::var(constants::ENV_GUILD_ID);

        if unparsed_guild_id.is_err() {
            error!("Cannot find guild id in env vars.");
            return;
        }

        let parsed_guild_id = unparsed_guild_id.unwrap().parse();

        if parsed_guild_id.is_err() {
            error!("Cannot parse guild id.");
            return;
        }

        let guild_id = GuildId(
            parsed_guild_id.unwrap()
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| PingController::register(command))
        })
        .await
        .unwrap();

        debug!(
            "Added guild commands: {:#?}",
            commands
                .iter()
                .map(|command| command.name.clone())
                .collect::<String>()
        );
    }
}

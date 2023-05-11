use std::env;

use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error};
use serenity::{prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId, interaction::Interaction}};

use crate::{core::{application_context::ApplicationContext, controllers::{SlashCommandControllerTrait}, constants}, controllers::{ ping::PingController, babyfoot_match::BabyfootMatchController, babyfoot_last_ten::BabyfootLastTenController}};

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
                command.data.options
                    .iter()
                    .map(|opt| format!("{}: {:?}", opt.name, opt.value))
                    .collect::<Vec<String>>()
                    .join(", "),
                command.user.tag()
            );

            let result = match command.data.name.as_str() {
                "ping" => PingController::run(&command, &ctx, &self.application_context).await,
                "babyfoot_match"=> BabyfootMatchController::run(&command, &ctx, &self.application_context).await,
                "babyfoot_last_ten"=> BabyfootLastTenController::run(&command, &ctx, &self.application_context).await,
                _ => Err(anyhow!("Not implemented !")),
            };

            if let Err(error) = result {
                error!(
                    "Received invalid command interaction: {:?} ({:?}) by {:?} -> {:?}",
                    command.data.name,
                    command.data.options
                    .iter()
                    .map(|opt| format!("{}: {:?}", opt.name, opt.value))
                    .collect::<Vec<String>>()
                    .join(", "),                    
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

        let registration_result = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| PingController::register(command))
                .create_application_command(|command| BabyfootMatchController::register(command))
                .create_application_command(|command| BabyfootLastTenController::register(command))
        })
        .await;

        if let Err(error) = &registration_result {
            error!("Error while registering commands: {:#?}", error);
            return;
        }

        debug!(
            "Added guild commands: {:#?}",
            registration_result
                .unwrap()
                .iter()
                .map(|command| command.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

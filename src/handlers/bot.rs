use std::env;

use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error};
use serenity::{
    model::prelude::{interaction::Interaction, GuildId, Ready},
    prelude::{Context, EventHandler},
};

use crate::{
    controllers::{
        babyfoot_match_1v1::BabyfootMatch1v1Controller, ping::PingController, babyfoot_match_2v2::BabyfootMatch2v2Controller, babyfoot_match_1v2::BabyfootMatch1v2Controller, babyfoot_player_stats::BabyfootPlayerStatsController, babyfoot_leaderboards::BabyfootLeaderboardsController,
    },
    core::{
        application_context::ApplicationContext, constants,
        controllers::SlashCommandControllerTrait,
    }, libraries::discord, renderers::command_renderer,
};

pub struct Bot {
    application_context: ApplicationContext,
}

impl Bot {
    pub fn new(application_context: ApplicationContext) -> Self {
        Self {
            application_context,
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!(
                "Received command interaction: {:?} ({:?}) by {:?}",
                command.data.name,
                command
                    .data
                    .options
                    .iter()
                    .map(|opt| format!("{}: {:?}", opt.name, opt.value))
                    .collect::<Vec<String>>()
                    .join(", "),
                command.user.tag()
            );

            let result = match command.data.name.as_str() {
                "ping" => PingController::run(&command, &ctx, &self.application_context).await,
                "babyfoot_1v1" => {
                    BabyfootMatch1v1Controller::run(&command, &ctx, &self.application_context).await
                },
                "babyfoot_2v2" => {
                    BabyfootMatch2v2Controller::run(&command, &ctx, &self.application_context).await
                },
                "babyfoot_1v2" => {
                    BabyfootMatch1v2Controller::run(&command, &ctx, &self.application_context).await
                },
                "babyfoot_player_stats" => {
                    BabyfootPlayerStatsController::run(&command, &ctx, &self.application_context).await
                },
                "babyfoot_leaderboards" => {
                    BabyfootLeaderboardsController::run(&command, &ctx, &self.application_context).await
                },
                _ => Err(anyhow!("Not implemented !")),
            };

            if let Err(error) = result {
                if let Ok(message) = command_renderer::error() {
                    discord::respond_with_message(&command, &ctx, message).await.unwrap_or(());
                }

                error!(
                    "Received invalid command interaction: {:?} ({:?}) by {:?} -> {:?}",
                    command.data.name,
                    command
                        .data
                        .options
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

        let guild_id = GuildId(parsed_guild_id.unwrap());

        let registration_result =
            GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| PingController::register(command))
                    .create_application_command(|command| {
                        BabyfootMatch1v1Controller::register(command)
                    })
                    .create_application_command(|command| {
                        BabyfootMatch2v2Controller::register(command)
                    })
                    .create_application_command(|command| {
                        BabyfootMatch1v2Controller::register(command)
                    })
                    .create_application_command(|command| {
                        BabyfootPlayerStatsController::register(command)
                    })
                    .create_application_command(|command| {
                        BabyfootLeaderboardsController::register(command)
                    })
            })
            .await;

        if let Err(error) = &registration_result {
            error!("Error while registering commands: {:#?}", error);
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

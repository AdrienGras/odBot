use std::env;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction, ChannelId,
    },
    prelude::Context,
};

use crate::{
    core::{application_context::ApplicationContext, controllers::SlashCommandControllerTrait, constants},
    libraries::discord,
    middlewares::babyfoot_middleware::BabyfootMiddleware, renderers:: command_renderer,
};

pub struct BabyfootPlayerStatsController;

#[async_trait]
impl SlashCommandControllerTrait for BabyfootPlayerStatsController {
    async fn run(
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        app: &ApplicationContext,
    ) -> Result<()> {
        debug!("Entering babyfoot player stats controller...");
        let options = &command.data.options;

        let j = discord::resolve_user_arg(options.get(0).unwrap())?;

        let middleware = BabyfootMiddleware::new(app);

        debug!("Executing BabyfootMiddleware::player_stat middleware...");
        let content = middleware
            .player_stat(j)
            .await?;

        debug!("Sending message on babyfoot channel...");
        let babyfoot_channel_id: String = env::var(constants::BABYFOOT_CHANNEL_ID)?.parse()?;
        let babyfoot_channel = ChannelId(babyfoot_channel_id.parse()?);
        discord::post_message_on_channel(command, ctx, babyfoot_channel, content).await?;

        debug!("Responding to sender...");
        Ok(discord::respond_with_message(command, ctx, command_renderer::success()?).await?)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("babyfoot_player_stats")
            .description("Récupérer les statistiques d'un joueur")
            .create_option(|option| {
                option
                    .name("player")
                    .description("Joueur")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }
}

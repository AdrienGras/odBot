use std::env;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
         interaction::application_command::ApplicationCommandInteraction, ChannelId,
    },
    prelude::Context,
};

use crate::{
    core::{application_context::ApplicationContext, controllers::SlashCommandControllerTrait, constants},
    libraries::discord,
    middlewares::babyfoot_middleware::BabyfootMiddleware, renderers:: command_renderer,
};

pub struct BabyfootLeaderboardsController;

#[async_trait]
impl SlashCommandControllerTrait for BabyfootLeaderboardsController {
    async fn run(
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        app: &ApplicationContext,
    ) -> Result<()> {
        debug!("Entering babyfoot leaderboards controller...");

        let middleware = BabyfootMiddleware::new(app);

        debug!("Executing BabyfootMiddleware::player_stat middleware...");
        let content = middleware
            .leaderboards()
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
            .name("babyfoot_leaderboards")
            .description("Remonte les leaderboards")
    }
}

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
    middlewares::babyfoot_middleware::BabyfootMiddleware, renderers::babyfoot_renderer,
};

pub struct BabyfootMatch1v2Controller;

#[async_trait]
impl SlashCommandControllerTrait for BabyfootMatch1v2Controller {
    async fn run(
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        app: &ApplicationContext,
    ) -> Result<()> {
        debug!("Entering babyfoot match 1v2 controller...");
        let options = &command.data.options;

        let j1 = discord::resolve_user_arg(options.get(0).unwrap())?;
        let score_j1: i64 = discord::resolve_int_arg(options.get(1).unwrap())?;

        let j3 = discord::resolve_user_arg(options.get(2).unwrap())?;
        let j4 = discord::resolve_user_arg(options.get(3).unwrap())?;
        let score_j3j4 = discord::resolve_int_arg(options.get(4).unwrap())?;

        let middleware = BabyfootMiddleware::new(app);

        debug!("Executing BabyfootMiddleware::register_match_1v2 middleware...");
        let content = middleware
            .register_match_1v2(j1, j3, j4, i32::try_from(score_j1)?, i32::try_from(score_j3j4)?)
            .await?;

        debug!("Sending message on babyfoot channel...");
        let babyfoot_channel_id: String = env::var(constants::BABYFOOT_CHANNEL_ID)?.parse()?;
        let babyfoot_channel = ChannelId(babyfoot_channel_id.parse()?);
        discord::post_message_on_channel(command, ctx, babyfoot_channel, content).await?;

        debug!("Responding to sender...");
        Ok(discord::respond_with_message(command, ctx, babyfoot_renderer::render_saved_match()?).await?)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("babyfoot_1v2")
            .description("Enregistrer un match de babyfoot en 1v2")
            .create_option(|option| {
                option
                    .name("j1")
                    .description("Premier joueur (joueur solo)")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("score-j1")
                    .description("Score du premier joueur (joueur solo)")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("j3")
                    .description("Premier joueur du second groupe")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("j4")
                    .description("Second joueur du second groupe")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("score-j3-j4")
                    .description("Score du second groupe")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
    }
}

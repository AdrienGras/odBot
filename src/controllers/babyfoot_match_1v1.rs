use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};

use crate::{
    core::{application_context::ApplicationContext, controllers::SlashCommandControllerTrait},
    libraries::discord,
    middlewares::babyfoot_middleware::BabyfootMiddleware,
};

pub struct BabyfootMatch1v1Controller;

#[async_trait]
impl SlashCommandControllerTrait for BabyfootMatch1v1Controller {
    async fn run(
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        app: &ApplicationContext,
    ) -> Result<()> {
        debug!("Entering babyfoot match 1v1 controller...");

        let options = &command.data.options;

        let j1 = discord::resolve_user_arg(options.get(0).unwrap())?;
        let score_j1: i64 = discord::resolve_int_arg(options.get(1).unwrap())?;

        let j2 = discord::resolve_user_arg(options.get(2).unwrap())?;
        let score_j2 = discord::resolve_int_arg(options.get(3).unwrap())?;

        let middleware = BabyfootMiddleware::new(app);

        debug!("Executing BabyfootMiddleware::register_match middleware...");
        let content = middleware
            .register_match_1v1(j1, j2, i32::try_from(score_j1)?, i32::try_from(score_j2)?)
            .await?;

        debug!("Responding to ping...");
        Ok(discord::respond_with_message(command, ctx, content).await?)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("babyfoot_1v1")
            .description("Enregistrer un match de babyfoot en 1v1 dans la base")
            .create_option(|option| {
                option
                    .name("j1")
                    .description("Premier joueur du match")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("score-j1")
                    .description("Score du premier joueur")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("j2")
                    .description("Second joueur du match")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("score-j2")
                    .description("Score du second joueur")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
    }
}

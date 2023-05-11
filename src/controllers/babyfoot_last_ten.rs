use anyhow::{Result};
use async_trait::async_trait;
use log::debug;
use serenity::{model::{prelude::{interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, command::CommandOptionType}, user::User}, builder::CreateApplicationCommand, prelude::Context};

use crate::{core::{ application_context::ApplicationContext, controllers::SlashCommandControllerTrait, middlewares::ActionMiddleware}, middlewares::{ping_middleware::PingMiddleware, babyfoot_middleware::BabyfootMiddleware}, libraries::discord};

pub struct BabyfootLastTenController;

#[async_trait]
impl SlashCommandControllerTrait for BabyfootLastTenController {
    async fn run(command: &ApplicationCommandInteraction, ctx: &Context, app: &ApplicationContext) -> Result<()> {
        debug!("Entering babyfoot last ten controller...");

        let middleware = BabyfootMiddleware::new(&app);

        debug!("Executing BabyfootMiddleware::last_ten middleware...");
        let content = middleware.last_ten().await?;
        
        debug!("Responding to command...");
        Ok(discord::respond_with_message(command, ctx, content).await?)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("babyfoot_last_ten")
            .description("Voir les 10 derniers matchs de babyfoot")
    }
}
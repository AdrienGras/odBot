use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, builder::CreateApplicationCommand, prelude::Context};

use crate::{core::{ application_context::ApplicationContext, controllers::SlashCommandControllerTrait, middlewares::ActionMiddleware}, middlewares::ping_middleware::PingMiddleware, libraries::discord};

pub struct PingController;

#[async_trait]
impl SlashCommandControllerTrait for PingController {
    async fn run(command: &ApplicationCommandInteraction, ctx: &Context, _app: &ApplicationContext) -> Result<()> {
        debug!("Entering ping controller...");

        let middleware = PingMiddleware::new(&command.user);
        debug!("Executing ping middleware...");
        let content = middleware.run().await?;
        
        debug!("Responding to ping...");
        Ok(discord::respond_with_message(command, ctx, content).await?)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}
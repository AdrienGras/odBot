use anyhow::Result;
use async_trait::async_trait;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use super::application_context::ApplicationContext;

#[async_trait]
pub trait SlashCommandControllerTrait {
    async fn run(command: &ApplicationCommandInteraction, ctx: &Context, app: &ApplicationContext)  -> Result<()>;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}
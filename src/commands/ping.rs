use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::user::User;
use serenity::utils::MessageBuilder;

use super::command_trait::DiscordSlashCommand;

#[derive(Default)]
pub struct PingCommand;

impl DiscordSlashCommand for PingCommand {
    fn run(options: &[CommandDataOption], user: &User) -> String {
        MessageBuilder::new()
            .push_quote("Hello ")
            .mention(user)
            .push(" it ")
            .push_bold("works !")
            .build()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}

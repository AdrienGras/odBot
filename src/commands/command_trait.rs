use serenity::{
    builder::CreateApplicationCommand,
    model::{prelude::interaction::application_command::CommandDataOption, user::User},
};

pub trait DiscordSlashCommand {
    /// runs the command 'entrypoint'.
    fn run(_options: &[CommandDataOption], emmiter: &User) -> String;

    /// configures the command to the server, adding name, description and parameters.
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

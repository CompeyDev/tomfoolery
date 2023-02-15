use serenity::{model::prelude::interaction::application_command::CommandDataOption, builder::CreateApplicationCommand};

pub fn run(_options: &[CommandDataOption]) -> String {
    return "pong!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    return command.name("ping").description("Oooh, what do this do?!")
}
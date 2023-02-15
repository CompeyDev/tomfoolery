mod commands;

use std::env;
use serenity::{
    prelude::*, 
    async_trait, 
    model::prelude::*, 
    model::application::interaction::{InteractionResponseType, Interaction}
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Slash command interaction {:#?} received", command);

            let reply = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "Requested command is non-existent!".to_string(),
            };

            if let Err(error) = command
                .create_interaction_response(&ctx.http, |res| {
                    res
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(reply))
                })
                .await
                {
                    println!("Failed to respond to slash command interaction with error: {:#?}", error);
                }
        }
    }

    async fn ready(&self, ctx: Context, client: Ready) {
        println!("Signed in as {}", client.user.name);

        let guild_id = GuildId(
            env::var("DISCORD_GUILD_ID")
                .expect("GUILD_ID environment variable not found!")
                .parse()
                .expect("GUILD_ID is not of a valid type (integer)"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
        })
        .await;

        println!("Registered application commands: {:#?}", commands)
    }
}


#[tokio::main]
pub async fn start() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable not found!");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Failed to build client");
    
    if let Err(error) = client.start().await {
        println!("Client failed to connect to gateway with error: {:#?}", error);
    }
}
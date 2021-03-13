mod role;
mod common;
mod questionnaire;

use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::common::command;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_name = common::channel::get_channel_name(&ctx,&msg).await;
        println!("channelIs:{}", channel_name);
        if common::channel::is_target_channel(channel_name).await {
            let command_and_type = common::command::distinction_command(&msg.content).await;
            match command_and_type{
                command::TypeOfCommand::Role(command)
                    => role::role_setting(&ctx,&msg,&command).await,
                command::TypeOfCommand::Questionnaire(command)
                    => questionnaire::create_questionnaire(&ctx,&msg,&command).await,
                command::TypeOfCommand::UnknownCommand
                    => println!("UnknownCommand"),
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}

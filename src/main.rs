use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[macro_use]
extern crate scan_fmt;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_name = get_channel_name(&ctx,&msg).await;
        println!("channelIs:{}", channel_name);

        let add_role_command = "i-want-an-announcement";
        let remove_role_command = "i-don't-want-an-announcement";
        let role_str = "RM-announce";
        
        if is_target_channel(channel_name).await {
            let command_string = &msg.content;
            println!("command is:{}", command_string);
            
            if command_string.starts_with(add_role_command){
                post_message(&ctx,&msg,"roleふってあげたいね".to_string()).await;
            }
            else if command_string.starts_with(remove_role_command){
                post_message(&ctx,&msg,"role外してあげたいね".to_string()).await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn have_a_role() -> bool {
    return false;
}

async fn add_role() -> bool {
    return false;
}

async fn remove_role() -> bool {
    return false;
}

async fn post_message(ctx: &Context, msg: &Message, message_str: String) {
    let response = MessageBuilder::new()
    .push(message_str)
    .build();
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
}

async fn get_channel_name(ctx: &Context, msg: &Message) -> String {
    let channel_name = match msg.channel_id.to_channel(&ctx).await{
        Ok(channel) =>channel,
        Err(why) =>{
            println!("Error:{:?}",why);
            return "".to_string();
        },
    };
    return channel_name.to_string();
}

async fn is_target_channel(channel_name: String) -> bool{
    let target_channel = env::var("TARGET_CHANNEL")
        .expect("Expected a target channnel");
    if channel_name != target_channel {
        println!("This is not target channel:{}",channel_name);
        return false;
    }
    return true;
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

pub mod post{
    pub async fn post_message(ctx: &crate::Context, msg: &crate::Message, message_str: String) {
        let response = crate::MessageBuilder::new()
        .push(message_str)
        .build();
        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub mod channel{
    pub async fn get_channel_name(ctx: &crate::Context, msg: &crate::Message) -> String {
        let channel_name = match msg.channel_id.to_channel(&ctx).await{
            Ok(channel) =>channel,
            Err(why) =>{
                println!("Error:{:?}",why);
                return "".to_string();
            },
        };
        return channel_name.to_string();
    }
    
    pub async fn is_target_channel(channel_name: String) -> bool {
        let target_channel = crate::env::var("TARGET_CHANNEL")
            .expect("Expected a target channnel");
        if channel_name != target_channel {
            println!("This is not target channel:{}",channel_name);
            return false;
        }
        return true;
    }
    
}

pub mod command{
    pub async fn distinction_command(command: &String) -> crate::TypeOfCommand {
        if command.starts_with("!role-"){
            println!("Role command");
            return crate::TypeOfCommand::Role(command[6..].to_string());
        }
        if command.starts_with("!questionnaire-"){
            println!("Questionnaire command");
            return crate::TypeOfCommand::Questionnaire(command[15..].to_string());
        }
        return crate::TypeOfCommand::UnknownCommand;
    }
}

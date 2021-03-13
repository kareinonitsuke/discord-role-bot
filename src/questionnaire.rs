pub use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
    builder::{EditMember,EditRole}
};

use crate::common::post;

pub enum TypeOfReaction{
    t_create_questionnaire(u8,u8,u8),
    emoji,
    noaction,
}

pub async fn create_questionnaire(ctx: &Context, msg: &Message, role_string: &String){
    let questionnaire_command = "rmq-";
    
    //println!("command is:{}", role_string);
    
    if  role_string.starts_with(questionnaire_command){
        //post::post_message(&ctx,&msg,"RM用にアンケート作ってあげたいね".to_string()).await;
        let result = rmq_distinction_command(&role_string[4..]);
        match result{
            TypeOfReaction::t_create_questionnaire(length, start_month, start_date)
                => acreate_questionnaire(&ctx,&msg,length, start_month, start_date).await, 
            TypeOfReaction::emoji
                => emoji(&ctx,&msg).await,
            _ => rm_error(&ctx,&msg).await,
        }
    }
    else{
        rm_error(&ctx,&msg).await;
    }
}

async fn acreate_questionnaire(ctx: &Context, msg: &Message, length:u8, start_month:u8, start_date:u8){
    let message = format!("!questionnaire-rmq-emoji\n{}月{}日からの一週間のアンケートをとりまーす", start_month, start_date);
    match length{
        0=> rm_error(&ctx,&msg).await,
        5=> post::post_message(&ctx,&msg,message.to_string()).await,
        _=> rm_error(&ctx,&msg).await,
    }
}

async fn emoji(ctx: &Context, msg: &Message){
    post::post_message(&ctx,&msg,"emojiつけたい".to_string()).await
}

fn rmq_distinction_command(command_str:&str) -> TypeOfReaction {
    println!("command is:{}", command_str);
    if let Ok((month,date)) = scan_fmt!(command_str, "1w-{d}-{d}", u8, u8){
        return TypeOfReaction::t_create_questionnaire(5, month, date);
    }
    if command_str.starts_with("emoji"){
        return TypeOfReaction::emoji;
    }
    return TypeOfReaction::noaction;
}

async fn rm_error(ctx: &Context, msg: &Message){
    post::post_message(&ctx,&msg,"RM用のなにをすればよいでしょう".to_string()).await
}
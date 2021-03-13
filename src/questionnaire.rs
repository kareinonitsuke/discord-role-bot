pub use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
    builder::{EditMember,EditRole}
};

use crate::common::post;
pub struct QuDetails{
    length:u8,
    start_date:(u8,u8),
}

pub async fn create_questionnaire(ctx: &Context, msg: &Message, role_string: &String){
    let questionnaire_command = "rmq";
    
    println!("command is:{}", role_string);
    
    if  role_string.starts_with(questionnaire_command){
        post::post_message(&ctx,&msg,"RM用にアンケート作ってあげたいね".to_string()).await;
    }
    else{
        post::post_message(&ctx,&msg,"RM用のなにをすればよいでしょう".to_string()).await;
    }

}


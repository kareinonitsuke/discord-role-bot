pub async fn role_setting(ctx: &crate::Context, msg: &crate::Message, role_string: &String) {
    let add_role_command = "i-want-an-announcement";
    let remove_role_command = "i-don't-want-an-announcement";
    let role = "RM-announce";
    
    println!("command is:{}", role_string);
    
    if role_string.starts_with(add_role_command){
        crate::common::post::post_message(&ctx,&msg,"roleふってあげたいね".to_string()).await;
    }
    else if role_string.starts_with(remove_role_command){
        crate::common::post::post_message(&ctx,&msg,"role外してあげたいね".to_string()).await;
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


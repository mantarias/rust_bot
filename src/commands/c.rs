//! echoes back the message sent to it
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[allow(unused_variables)]
#[command]
async fn c(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut response = String::new();
    let mut i = 1;

    while !args.is_empty() {
        if let Some(arg) = args.single_quoted::<String>().ok() {
            response += &format!("arg{} = {}\n", i, arg);
            i += 1;
        }
    }

    if !response.is_empty() {
        msg.reply(ctx, response).await?;
    }

    Ok(())
}

//! echoes back the message sent to it
use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[allow(unused_variables)]
#[command]
async fn c(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split_whitespace().collect();


    Ok(())
}

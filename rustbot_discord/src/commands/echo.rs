//! echoes back the message sent to it
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    let content = &msg.content.replacen("-echo", "", 1);
    if content.trim().is_empty() {
        msg.reply(ctx, "You didn't provide any text to echo. This command works by typing `-echo` followed by the message you want echoed. For example, typing `-echo Hello World!` will cause me to respond with `Hello World!`").await?;
    } else {
        msg.reply(ctx, content).await?;
    }
    Ok(())
}

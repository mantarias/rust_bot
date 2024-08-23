//! echoes back the message sent to it
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn react(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "this isnt programmed yet!").await?;
    Ok(())
}

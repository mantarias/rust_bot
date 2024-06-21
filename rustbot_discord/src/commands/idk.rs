//! This module contains the `idk` command.
//!
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn idk(ctx: &Context, msg: &Message) -> CommandResult {
    // Record the time when the command was received.

    // Send a message to indicate that the bot is calculating latency.
    msg.channel_id.say(&ctx.http, "Pinging...").await?;

    // Calculate the round-trip latency.

    Ok(())
}

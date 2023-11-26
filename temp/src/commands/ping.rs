//! This module contains the `ping` command.
//!
//! The `ping` command simply calculates the round-trip latency between the bot and the Discord API.
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Instant;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Record the time when the command was received.
    let start_time = Instant::now();

    // Send a message to indicate that the bot is calculating latency.
    let mut response = msg.channel_id.say(&ctx.http, "Pinging...").await?;

    // Calculate the round-trip latency.
    let end_time = Instant::now();
    let latency = end_time.duration_since(start_time);

    // Edit the response message with latency information.
    response
        .edit(&ctx.http, |m| {
            m.content(format!("Pong! Round-trip latency: {:?}", latency))
        })
        .await?;

    Ok(())
}

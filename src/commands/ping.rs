//! A simple Discord bot command using the Serenity library.
//!
//! This module demonstrates the implementation of a basic "ping" command.
//! When the command is invoked in a Discord channel, the bot responds with "Pong!".

// Importing necessary modules and types from the Serenity library.
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

/// Processes the "ping" command.
///
/// When this command is invoked in a Discord channel, the bot will respond with "Pong!".
/// This function is asynchronous and will await the response operation.
///
/// # Arguments
///
/// * `ctx` - The context of the command, which provides data and methods needed to interact
///           with Discord.
/// * `msg` - The message that triggered this command.
///
/// # Returns
///
/// This function returns a `CommandResult`, which is an alias for the `Result` type.
/// It will be `Ok` if the command processed successfully, or an error type if it failed.
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Sending a reply to the message that triggered the command.
    msg.reply(ctx, "Pong!").await?;

    // Indicating successful execution of the command.
    Ok(())
}

//! check if a site is alive
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

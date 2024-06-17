use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn ustatus(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

//! # fix
use serenity::model::id::MessageId;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
#[command]
async fn fix(ctx: &Context, msg: &Message) -> CommandResult {
    // create a message to a message with id "1176898431319543930"
    msg.reply(ctx, "Think something can be improved? Make a bug report or work on it yourself [here](https://github.com/mantarias/rust_bot)").await?;

    Ok(())
}

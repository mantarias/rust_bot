use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.content.replacen("-echo", "", 1))
        .await?;
    Ok(())
}

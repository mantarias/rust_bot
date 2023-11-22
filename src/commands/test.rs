use serenity::model::id::MessageId;
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.id).await?;
    // create a message to a message with id "1176898431319543930"
    let channel_id = msg.channel_id;
    let message_id = MessageId(1176898431319543930);
    let message = channel_id.message(&ctx.http, message_id).await?;
    message.reply(ctx, "content").await?;
    msg.reply(ctx, &message.content).await?;

    Ok(())
}
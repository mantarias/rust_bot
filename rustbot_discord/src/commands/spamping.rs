use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::UserId},
    prelude::*,
};
use tokio::time::Duration;

#[command]
#[num_args(1)]
async fn spamping(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mentioned_user_id: UserId = args.single::<UserId>()?;

    let new_message = msg.channel_id.say(ctx, format!("Thread for {}", mentioned_user_id.0)).await?;
    let channel = msg.channel_id.create_public_thread(ctx, new_message.id, |m| {
        m.name("New Thread")
    }).await?;

    for _ in 0..1000 {
        channel.send_message(ctx, |m| {
            m.content(format!("<@{}>", mentioned_user_id.0))
        }).await?;
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

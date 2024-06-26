use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            // Check if the user is the bot itself
            if user.id == ctx.cache.current_user_id().await {
                msg.channel_id
                    .say(&ctx.http, "I can't kick myself!")
                    .await?;
                return Ok(());
            }

            if let Ok(member) = msg.guild_id.unwrap().member(&ctx.http, user.id).await {
                if member.permissions(&ctx.http).await?.administrator() {
                    msg.channel_id
                        .say(&ctx.http, "I can't kick the owner of the server!")
                        .await?;
                    return Ok(());
                }
            }

            if let Some(reason) = msg.content.split(' ').nth(1) {
                if let Err(why) = msg.guild_id.unwrap().kick_with_reason(&ctx.http, user, reason).await {
                    println!("Error kicking user: {:?}", why);
                    msg.channel_id
                        .say(&ctx.http, "Error kicking the user, please ask the developers to check logs")
                        .await?;
                } else {
                    msg.channel_id
                        .say(&ctx.http, format!("Kicked {} for: {}", user.name, reason))
                        .await?;
                }
            } else {
                msg.channel_id
                    .say(
                        &ctx.http,
                        "Please provide a reason for the kick. For example: '!kick @username <reason>'",
                    )
                    .await?;
            }
        }
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "Please mention a user to kick",
            )
            .await?;
    }

    Ok(())
}

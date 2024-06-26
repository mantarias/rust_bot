use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            // Check if the user is the bot itself
            if user.id == ctx.cache.current_user_id().await {
                msg.channel_id
                    .say(&ctx.http, "I can't ban myself!")
                    .await?;
                return Ok(());
            }

            if let Ok(member) = msg.guild_id.unwrap().member(&ctx.http, user.id).await {
                if member.permissions(&ctx.http).await?.administrator() {
                    msg.channel_id
                        .say(&ctx.http, "I can't ban the owner of the server!")
                        .await?;
                    return Ok(());
                }
            }

            if let Some(reason) = msg.content.split(' ').nth(1) {
                if let Err(why) = msg.guild_id.unwrap().ban_with_reason(&ctx.http, user, 7, reason).await {
                    println!("Error banning user: {:?}", why);
                    msg.channel_id
                        .say(&ctx.http, "Error banning the user, please ask the developers to check logs")
                        .await?;
                } else {
                    msg.channel_id
                        .say(&ctx.http, format!("Banned {} for: {}", user.name, reason))
                        .await?;
                }
            } else {
                msg.channel_id
                    .say(
                        &ctx.http,
                        "Please provide a reason for the ban. For example: '!ban @username <reason>'",
                    )
                    .await?;
            }
        }
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "Please mention a user to ban",
            )
            .await?;
    }

    Ok(())
}

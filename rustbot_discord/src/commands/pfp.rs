//! Returns the profile pictures of the pinged users.
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn pfp(ctx: &Context, msg: &Message) -> CommandResult {
    // Check if any user mentions were provided in the command.
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            // Get each user's profile picture URL.
            if let Some(avatar_url) = user.avatar_url() {
                // Send the profile picture URL as a reply.
                msg.channel_id.say(&ctx.http, &avatar_url).await?;
            } else {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!("{} does not have a profile picture.", user.name),
                    )
                    .await?;
            }
        }
    } else {
        // Providing usage and example if no user was mentioned
        msg.channel_id
            .say(
                &ctx.http,
                "Please mention a user to get their profile picture. For example: '!pfp @username'",
            )
            .await?;
    }

    Ok(())
}

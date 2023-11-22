use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

#[command]
async fn pfp(ctx: &Context, msg: &Message) -> CommandResult {
    // Check if a user mention was provided in the command.
    if let Some(user) = msg.mentions.get(0) {
        // Get the user's profile picture URL.
        if let Some(avatar_url) = user.avatar_url() {
            // Send the profile picture URL as a reply.
            msg.channel_id.say(&ctx.http, &avatar_url).await?;
        } else {
            msg.channel_id.say(&ctx.http, "User does not have a profile picture.").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "Please mention a user to get their profile picture.").await?;
    }

    Ok(())
}

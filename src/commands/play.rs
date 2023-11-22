use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use songbird::{
    input::ytdl,
    tracks::PlayMode,
};
use std::sync::Arc;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Check if the user is in a voice channel.
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "This command can only be used in a server with voice channels.").await?;
            return Ok(());
        }
    };

    // Get the user's voice channel.
    let voice_state = match msg.author.voice_channel(&ctx.cache) {
        Some(state) => state,
        None => {
            msg.channel_id.say(&ctx.http, "You must be in a voice channel to use this command.").await?;
            return Ok(());
        }
    };

    // Connect to the user's voice channel.
    let manager = songbird::get(&ctx).await.unwrap();
    let handler_lock = manager.join(guild_id, voice_state.channel_id).await;

    if let Some(mut handler) = handler_lock {
        // Parse the YouTube URL from command arguments.
        let url = args.rest();

        // Load the audio track from the YouTube URL.
        let source = ytdl(url).await?;

        // Play the track.
        handler.play_only_source(source, PlayMode::default()).await?;
    }

    Ok(())
}

//! plays a song from a YouTube URL
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use songbird::input::ytdl;

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Received play command");

    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => {
            println!("Guild found");
            guild
        }
        None => {
            println!("Play command is not in a server with voice channels.");
            msg.channel_id
                .say(
                    &ctx.http,
                    "This command can only be used in a server with voice channels.",
                )
                .await?;
            return Ok(());
        }
    };

    let author_id = msg.author.id;

    let voice_state = match guild.voice_states.get(&author_id) {
        Some(state) => {
            println!("User's voice state found");
            state.clone()
        }
        None => {
            println!("User is not in a voice channel.");
            msg.channel_id
                .say(
                    &ctx.http,
                    "You must be in a voice channel to use this command.",
                )
                .await?;
            return Ok(());
        }
    };

    let manager = match songbird::get(ctx).await {
        Some(manager) => {
            println!("Songbird manager obtained");
            manager
        }
        None => {
            println!("Failed to obtain Songbird manager");
            return Ok(());
        }
    };

    let (handler_lock, success) = manager
        .join(guild.id, voice_state.channel_id.unwrap())
        .await;

    if let Err(e) = success {
        println!("Error joining the voice channel: {:?}", e);
        msg.channel_id
            .say(
                &ctx.http,
                format!("Error joining the voice channel: {:?}", e),
            )
            .await?;
        return Ok(());
    }

    println!("Joined the voice channel");

    let mut handler = handler_lock.lock().await;

    let url = msg.content.replace("-play ", "");
    println!("URL extracted: {}", url);

    match ytdl(url).await {
        Ok(source) => {
            println!("YouTube source obtained");
            let _track_handle = handler.play_only_source(source);
            println!("Track is now playing");
        }
        Err(e) => {
            println!("Error loading YTDL source: {:?}", e);
            msg.channel_id
                .say(&ctx.http, format!("Error loading YTDL source: {:?}", e))
                .await?;
        }
    }

    Ok(())
}

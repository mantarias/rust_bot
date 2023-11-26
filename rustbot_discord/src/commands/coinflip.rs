//! Command for flipping a coin.

use rand::Rng;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;

pub enum CoinflipResult {
    Heads,
    Tails,
}

impl CoinflipResult {
    fn as_str(&self) -> &str {
        match self {
            CoinflipResult::Heads => "Heads",
            CoinflipResult::Tails => "Tails",
        }
    }
}

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    let animation_frames = ["Coin is flipping...", "Still flipping..."];
    let mut animation_msg = msg.channel_id.say(&ctx.http, animation_frames[0]).await?;

    for &frame in animation_frames.iter().cycle().take(6) {
        tokio::time::sleep(Duration::from_secs(1)).await;
        animation_msg.edit(&ctx.http, |m| m.content(frame)).await?;
    }

    let result = if rand::thread_rng().gen::<bool>() {
        CoinflipResult::Heads
    } else {
        CoinflipResult::Tails
    };

    let response = result.as_str();

    animation_msg
        .edit(&ctx.http, |m| {
            m.content(format!("Coinflip result: {}", response))
        })
        .await?;

    Ok(())
}

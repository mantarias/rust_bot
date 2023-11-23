//! Command for flipping a coin.
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use rand::Rng;

// Enum to represent the result of the coinflip.
pub enum CoinflipResult {
    Heads,
    Tails,
}

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    // Simulate the coinflip.
    let result = if rand::thread_rng().gen::<bool>() {
        CoinflipResult::Heads
    } else {
        CoinflipResult::Tails
    };

    // Respond with the result.
    let response = match result {
        CoinflipResult::Heads => "Heads",
        CoinflipResult::Tails => "Tails",
    };

    msg.channel_id.say(&ctx.http, format!("Coinflip result: {}", response)).await?;

    Ok(())
}

//! This file is the main entry point for the bot. It sets up the bot and starts it.
//! Will print out a message if updated by a user.
use dotenv::dotenv;
use std::env;
use std::fmt::format;
use songbird::SerenityInit;

use serenity::model::id::{ChannelId, MessageId};
use serenity::{
    async_trait, framework::standard::StandardFramework, model::gateway::GatewayIntents, prelude::*,
};
use std::fs;
use std::str::FromStr;

mod commands;
mod web;

use tokio_postgres::{NoTls, Error};
use commands::GENERAL_GROUP;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost port=5432 dbname=rustbot password=Bean1! user=postgres", NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    // Start the web server in a separate async task
    tokio::spawn(async {
        web::run_server(client).await;
    });

    // Configure the client with the bot's prefix and commands
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| c.prefix("-")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .framework(framework) // framework is now used here after all configurations
        .await
        .expect("Error creating client");

    if let Ok(contents) = fs::read_to_string("update.txt") {
        let parts: Vec<&str> = contents.split_whitespace().collect();
        if parts.len() == 2 {
            let channel_id = ChannelId::from_str(parts[0]).expect("Invalid Channel ID");
            let message_id = MessageId::from_str(parts[1]).expect("Invalid Message ID");

            if let Ok(message) = channel_id
                .message(&client.cache_and_http.http, message_id)
                .await
            {
                match message
                    .reply(
                        &client.cache_and_http.http,
                        "Bot restarted successfully! ty for you waiting",
                    )
                    .await
                {
                    Ok(_) => fs::remove_file("update.txt").expect("Could not remove update.txt"),
                    Err(e) => println!("Error sending reply: {:?}", e),
                }
            }
        }
    }

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
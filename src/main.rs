use dotenv::dotenv;
use reqwest;
use serenity::builder::GetMessages;
use serenity::model::id::{ChannelId, MessageId};
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::GatewayIntents},
    prelude::*,
    utils::parse_message_id_pair,
};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs::File};

#[group]
#[commands(ping, echo, ustatus, status, stats, update, test)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
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
                message
                    .reply(&client.cache_and_http.http, "Bot restarted successfully! ty for you waiting")
                    .await;
                fs::remove_file("update.txt").expect("Could not remove update.txt");
            }
        }
    }
    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong!").await?;

    Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.content.replacen("-echo", "", 1))
        .await?;
    Ok(())
}

#[command]
async fn ustatus(ctx: &Context, msg: &Message) -> CommandResult {
    let res: String;

    match reqwest::get("https://uninotes.mantarias.com").await {
        Ok(response) => {
            if response.status().is_success() {
                res = "uninotes is alive".to_string();
            } else {
                res = format!("uninotes responded with status: {}", response.status());
            }
        }
        Err(_) => {
            res = "uninotes is not reachable".to_string();
        }
    }

    msg.reply(ctx, res).await?;
    Ok(())
}

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let res: String;
    let mut site = msg.content.replacen("-status ", "", 1);
    site = if site.starts_with("https") {
        site
    } else {
        format!("http://{}", site)
    };

    match reqwest::get(site.clone()).await {
        Ok(response) => {
            if response.status().is_success() {
                res = format!("{} is alive", site);
            } else {
                res = format!("{} responded with status: {}", site, response.status());
            }
        }
        Err(_) => {
            res = format!("{} is not reachable", site);
        }
    }

    msg.reply(ctx, res).await?;
    Ok(())
}

#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    channel_id.broadcast_typing(&ctx.http).await?;
    let mut last_message_id = None;
    let mut all_messages = Vec::new();
    // parse to int

    let count = msg.content.replacen("-stats ", "", 1);
    // part count to int with error handeling
    let count: u64 = match count.parse() {
        Ok(n) => n,
        Err(_) => {
            msg.reply(ctx, "Please enter a valid number").await?;
            return Ok(());
        }
    };
    let mut citter = 0;
    let limitt = if count < 100 {
        count % 100
    } else {
        citter = count / 100;
        100
    };
    let mut current_run: u64 = 0;

    loop {
        let messages = channel_id
            .messages(&ctx.http, |retriever| {
                retriever.limit(limitt); // Set to the maximum limit
                if let Some(message_id) = last_message_id {
                    retriever.before(message_id); // Fetch messages before the last one we got
                }
                retriever
            })
            .await?;

        // Break the loop if no more messages are fetched
        if messages.is_empty() {
            break;
        }

        // Save the ID of the last message in this batch for the next iteration with error handeling
        last_message_id = match messages.last() {
            Some(message) => Some(message.id),
            None => {
                msg.reply(ctx, "Something went wrong").await?;
                return Ok(());
            }
        };

        // Process and store messages
        for message in messages.iter() {
            all_messages.push(message.clone());
        }
        current_run += 1;
        if citter <= current_run {
            break;
        }
    }
    let mut i = 0;
    // create vector of users and count messages like this {user, count}
    let mut message_counts: HashMap<String, i32> = HashMap::new();

    for message in &all_messages {
        println!("{} {}: {}", i, message.author.name, message.content);
        i += 1;
        // if message.author.name is in message_counts add 1 to count else add user to message_counts
        if message_counts.contains_key(&message.author.name) {
            *message_counts
                .entry(message.author.name.clone())
                .or_insert(0) += 1;
        } else {
            message_counts.insert(message.author.name.clone(), 1);
        }
    }

    // Format, sort and save the best 10 users
    let mut output = String::new();
    let mut best_users = Vec::new();
    for (author, count) in message_counts.iter() {
        best_users.push((author, count));
    }
    best_users.sort_by(|a, b| b.1.cmp(a.1));
    for (author, count) in best_users.iter().take(10) {
        output.push_str(format!("{}: {}\n", author, count).as_str());
    }

    // Now all_messages contains more than the initial limit
    // You can process them as needed
    msg.reply(ctx, output).await?;
    Ok(())
}

#[command]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
    // Create a Command to represent the program to be executed
    msg.reply(ctx, "updating").await?;
    // save the message id to file with error handeling
    let mut file = match std::fs::File::create("update.txt") {
        Ok(file) => file,
        Err(_) => {
            msg.reply(ctx, "Something went wrong").await?;
            return Ok(());
        }
    };
    let save_data = format!("{} {}", msg.channel_id, msg.id);
    match file.write_all(save_data.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            msg.reply(ctx, "Something went wrong").await?;
            return Ok(());
        }
    }

    let mut command = Command::new("systemctl");

    // Add any arguments to the command
    command.arg("restart").arg("rustbot.service");

    // Execute the command, which returns a Result containing the child process
    match command.spawn() {
        Ok(mut child) => {
            // Wait for the command to complete and get the exit status, send message if error

            match child.wait() {
                Ok(status) => println!("Exited with status: {}", status),
                Err(e) => eprintln!("Failed to wait for command: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
    msg.reply(ctx, "something went wrong, check the logs or try again")
        .await?;
    Ok(())
}
#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.id).await?;
    // create a message to a message with id "1176898431319543930"
    let channel_id = msg.channel_id;
    let message_id = MessageId(1176898431319543930);
    let message = channel_id.message(&ctx.http, message_id).await?;
    message.reply(ctx, "content").await?;
    msg.reply(ctx, &message.content).await?;

    Ok(())
}

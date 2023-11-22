use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use std::collections::HashMap;
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

use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use std::collections::HashMap;

#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    channel_id.broadcast_typing(&ctx.http).await?;

    let requested_count: u64 = match msg.content.replacen("-stats ", "", 1).parse() {
        Ok(num) => num,
        Err(_) => {
            msg.reply(ctx, "Please enter a valid number").await?;
            return Ok(());
        }
    };

    let mut last_message_id = None;
    let mut all_messages = Vec::new();
    let mut total_fetched = 0;

    while total_fetched < requested_count {
        let fetch_count = std::cmp::min(requested_count - total_fetched, 100);
        let messages = channel_id
            .messages(&ctx.http, |retriever| {
                retriever.limit(fetch_count);
                if let Some(message_id) = last_message_id {
                    retriever.before(message_id);
                }
                retriever
            })
            .await?;

        if messages.is_empty() {
            break;
        }

        last_message_id = messages.last().map(|message| message.id);
        total_fetched += messages.len() as u64;
        all_messages.extend(messages);
    }

    let mut message_counts: HashMap<String, i32> = HashMap::new();
    for message in all_messages {
        *message_counts.entry(message.author.name).or_insert(0) += 1;
    }

    let mut best_users: Vec<_> = message_counts.into_iter().collect();
    best_users.sort_by(|a, b| b.1.cmp(&a.1));

    let output = best_users.into_iter()
        .take(10)
        .map(|(author, count)| format!("{}: {}\n", author, count))
        .collect::<String>();

    msg.reply(ctx, output).await?;

    Ok(())
}


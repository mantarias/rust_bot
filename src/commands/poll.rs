//! creates a poll with reactions
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use serenity::model::prelude::ReactionType;

#[command]
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    // Split the message content into two parts: the command and the rest.
    let split_content: Vec<&str> = msg.content.splitn(2, ' ').collect();
    if split_content.len() != 2 {
        msg.channel_id.say(&ctx.http, "Please provide a question and at least one option for the poll.").await?;
        return Ok(());
    }

    // Further split the second part into the question and options, using '?' as a delimiter.
    let question_and_options: Vec<&str> = split_content[1].splitn(2, '?').collect();
    if question_and_options.len() != 2 {
        msg.channel_id.say(&ctx.http, "Please provide a question followed by a '?' and then the options.").await?;
        return Ok(());
    }

    let question = question_and_options[0].trim();
    let options: Vec<&str> = question_and_options[1].split_whitespace().collect(); // Split the options by whitespace.

    // Check if at least one option was provided.
    if options.is_empty() {
        msg.channel_id.say(&ctx.http, "Please provide at least one option for the poll.").await?;
        return Ok(());
    }

    // Create the poll message with reactions for each option.
    let mut poll_message = format!("**Poll: {}?**\n\n", question);

    // Add reactions for each option.
    let emojis = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟"];
    for (i, option) in options.iter().enumerate() {
        if i < emojis.len() {
            poll_message.push_str(format!("{} {}\n", emojis[i], option).as_str());
        } else {
            poll_message.push_str(format!("{}\n", option).as_str());
        }
    }

    // Send the poll message.
    let poll_msg = msg.channel_id.say(&ctx.http, poll_message).await?;

    // Add reactions to the poll message.
    for i in 0..emojis.len().min(options.len()) {
        poll_msg.react(&ctx.http, ReactionType::Unicode(emojis[i].to_string())).await?;
    }

    Ok(())
}

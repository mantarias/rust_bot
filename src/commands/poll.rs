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
    // Remove the command itself and split the remaining content into options.
    let content = &msg.content;
    let options: Vec<&str> = content.splitn(11, " ").collect(); // Allowing up to 10 options.

    // Check if at least two options were provided (question and one option).
    if options.len() < 3 {
        msg.channel_id.say(&ctx.http, "Please provide a question and at least one option for the poll.").await?;
        return Ok(());
    }

    // Extract the question and options.
    let question = options[1];
    let options = &options[2..]; // Skip the command and question.

    // Create the poll message with reactions for each option.
    let mut poll_message = format!("**Poll: {}**\n\n", question);

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

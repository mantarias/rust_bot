use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::ChannelId, guild::Guild, channel::GuildChannel},
    prelude::*,
};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;



#[derive(Serialize, Deserialize)]
struct GuildInfo {
    id: String,
    name: String,
    icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ChannelInfo {
    id: String,
    category_id: Option<String>, // Corrected field name
    category: Option<String>,
    name: String,
    topic: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MessageInfo {
    id: String,
    author: String,
    timestamp: String,
    content: String,
    // Add more fields if needed
}

#[derive(Serialize, Deserialize)]
struct ChatExport {
    guild: GuildInfo,
    channel: ChannelInfo,
    messages: Vec<MessageInfo>,
    exported_at: String,
}

#[command]
pub async fn export_chat(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or("This command can only be used in a server.")?;
    let channel_id = msg.channel_id;

    let guild = guild_id.to_guild_cached(&ctx.cache).ok_or("Unable to access guild info.")?;
    let channel = channel_id.to_channel_cached(&ctx.cache).ok_or("Unable to access channel info.")?;
    let guild_channel = match channel {
        serenity::model::channel::Channel::Guild(c) => c,
        _ => return Err("Channel not found in guild.".into()),
    };

    let messages = channel_id.messages(&ctx.http, |retriever| retriever.limit(100)).await?;

    let chat_export = ChatExport {
        guild: GuildInfo {
            id: guild.id.0.to_string(),
            name: guild.name.clone(), // Clone the name here
            icon_url: guild.icon_url(),
        },
        channel: ChannelInfo {
            id: guild_channel.id.0.to_string(),
            category_id: guild_channel.parent_id.map(|id| id.0.to_string()),
            category: guild_channel.parent_id.and_then(|id| id.to_channel_cached(&ctx.cache)).and_then(|c| match c {
                serenity::model::channel::Channel::Category(c) => Some(c.name),
                _ => None,
            }),
            name: guild_channel.name,
            topic: guild_channel.topic,
        },
        messages: messages.into_iter().map(|m| MessageInfo {
            id: m.id.0.to_string(),
            author: m.author.name,
            timestamp: m.timestamp.to_rfc3339(),
            content: m.content,
        }).collect(),
        exported_at: chrono::Utc::now().to_rfc3339(),
    };

    let json_string = serde_json::to_string_pretty(&chat_export)?;

    let mut file = File::create("chat_export.json")?;
    file.write_all(json_string.as_bytes())?;

    msg.channel_id.say(&ctx.http, "Chat exported successfully.").await?;

    Ok(())
}

//! Commands to convert text to "uwu" language.
use rand::seq::SliceRandom;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    // Check if the message is a reply and if it starts with "-uwu".
    if let Some(reference) = msg.referenced_message.clone() {
        // Get the content of the replied message.
        let original_content = reference.content;

        // Convert the content to "uwu" language with random emoticons.
        let uwu_content = uwunize(&original_content);

        // Send the "uwu" version as a reply.
        msg.delete(&ctx.http).await?;
        msg.channel_id.say(&ctx.http, &uwu_content).await?;
    }

    Ok(())
}

// Function to convert text to "uwu" language with random emoticons.
fn uwunize(text: &str) -> String {
    let emoticons = vec![
        "(≧◡≦)",
        "(◕‿◕✿)",
        "(｡♥‿♥｡)",
        "(*≧ω≦)",
        "(>‿◠)✌",
        "(⁄ ⁄•⁄ω⁄•⁄ ⁄)⁄",
        "(´｡• ᵕ •｡`)",
        "(◠‿◠✿)",
        "(｡♥‿♥｡)",
        "(¬‿¬)",
        "(◕ᴥ◕)",
        "(づ｡◕‿‿◕｡)づ",
        "(´∩｡• ᵕ •｡∩`)",
        "(⁄ ⁄•⁄з⁄•⁄ ⁄)",
        "(≖ᴗ≖✿)",
        "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)",
        "( ˘ ³˘)",
        "(¬‿¬)",
        "(´• ω •`)",
        "(✪‿✪)ノ",
        "(｡♥‿♥｡)",
        "(⺣◡⺣)♡*",
        "(*≧▽≦)",
        "(¬‿¬)",
        "(◕ᴗ◕✿)",
        "( ´ ▽ ` )ﾉ",
        "✧◝(⁰▿⁰)◜✧",
        "☆*:.｡.o(≧▽≦)o.｡.:*☆",
        "(｡♥‿♥｡)✨",
        "(｡♥‿♥｡)🌟",
        "(⁄ ⁄•⁄ω⁄•⁄ ⁄)⁄ ✨",
        "(◕‿◕✿)✨",
        "(｡♥‿♥｡)💫",
        "(＾◡＾)っ✨",
        "(✪‿✪)ノ✨",
        "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)✨",
        "( ˘ ³˘)❤️✨",
        "(¬‿¬)✨",
        "(´• ω •`)✨",
        "(*≧▽≦)✨",
        "(¬‿¬)💖✨",
        "(ʘ‿ʘ)✨",
        "(≖ᴗ≖✿)✨",
        "(◠‿◠✿)✨",
        "(¬‿¬)🌟",
        "(*≧ω≦)✨",
    ];

    let mut uwu_text = String::new();
    let chars = text.chars().peekable();

    for c in chars {
        match c {
            '.' | ',' => {
                // Randomly select an emoticon.
                let emoticon = emoticons.choose(&mut rand::thread_rng()).unwrap();
                uwu_text.push_str(emoticon);
            }
            'r' => uwu_text.push('w'),
            'l' => uwu_text.push('w'),
            'R' => uwu_text.push('W'),
            'L' => uwu_text.push('W'),
            _ => uwu_text.push(c),
        }
    }

    uwu_text
}

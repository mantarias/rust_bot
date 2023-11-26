use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

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

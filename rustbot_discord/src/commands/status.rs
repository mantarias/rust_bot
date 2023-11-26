//! check if a site is alive
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

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

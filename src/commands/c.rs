use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use serenity::framework::standard::Args;
use tokio_postgres::NoTls;

use crate::MyClient;

#[command]
async fn c(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Extract first argument
    let arg = match args.single_quoted::<String>() {
        Ok(a) => a,
        Err(_) => return Err("You need to provide an argument.".into()),
    };
    let data = ctx.data.read().await;
    let client = data.get::<MyClient>().unwrap();

    // Query the "commands" table with the argument and gather the results.
    let rows = match client.query("SELECT * FROM commands WHERE command_name = $1", &[&arg]).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Query error: {}", err);
            return Err("An error occurred while querying the database.".into());
        }
    };

    // If argument matches a row in the table, reply with corresponding response.
    if let Some(row) = rows.get(0) {
        let response: String = row.get("command_response"); // Assuming table has a 'command_response' column
        msg.reply(ctx, response).await?; // Reply with the response
    } else {
        msg.reply(ctx, "The command doesn't match any of the rows in the table.").await?; // If argument doesn't match any
    }

    Ok(())
}
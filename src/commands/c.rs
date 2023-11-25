use rand::Rng;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;
use serenity::framework::standard::Args;
use tokio_postgres::NoTls;

#[command]
async fn c(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Extract first argument
    let arg = match args.single_quoted::<String>() {
        Ok(a) => a,
        Err(_) => return Err("You need to provide an argument.".into()),
    };

    // Connect to the database.
    let (client, connection) = match tokio_postgres::connect(
        "host=localhost port=5432 dbname=rustbot password=Bean1! user=postgres",
        NoTls,
    )
        .await {
            Ok((client, connection)) => (client, connection),
            Err(err) => {
                eprintln!("Connection error: {}", err);
                return Err("Unable to connect to the database.".into())
            }
        };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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
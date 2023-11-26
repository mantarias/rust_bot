//! updates the bot from github
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::io::Write;
use std::process::Command;

#[command]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
    // Create a Command to represent the program to be executed
    msg.reply(ctx, "updating").await?;
    // Save the message id to file with error handling
    let mut file = match std::fs::File::create("update.txt") {
        Ok(file) => file,
        Err(e) => {
            msg.reply(ctx, &format!("Failed to create file: {}", e))
                .await?;
            return Ok(());
        }
    };
    let save_data = format!("{} {}", msg.channel_id, msg.id);
    match file.write_all(save_data.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            msg.reply(ctx, &format!("Failed to write file: {}", e))
                .await?;
            return Ok(());
        }
    }

    let mut command = Command::new("systemctl");

    // Add any arguments to the command
    command.arg("restart").arg("rustbot.service");

    // Execute the command, which returns a Result containing the child process
    match command.spawn() {
        Ok(mut child) => {
            // Wait for the command to complete and get the exit status, send message if error
            match child.wait() {
                Ok(status) => println!("Exited with status: {}", status),
                Err(e) => {
                    eprintln!("Failed to wait for command: {}", e);
                    msg.reply(ctx, &format!("Failed to wait for command: {}", e))
                        .await?;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            msg.reply(ctx, &format!("Failed to execute command: {}", e))
                .await?;
        }
    }
    msg.reply(ctx, "something went wrong, check the logs or try again")
        .await?;
    Ok(())
}

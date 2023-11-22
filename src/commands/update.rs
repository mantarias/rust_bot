use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use std::process::Command;
use std::io::Write;


#[command]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
    // Create a Command to represent the program to be executed
    msg.reply(ctx, "updating").await?;
    // save the message id to file with error handeling
    let mut file = match std::fs::File::create("update.txt") {
        Ok(file) => file,
        Err(_) => {
            msg.reply(ctx, "Something went wrong").await?;
            return Ok(());
        }
    };
    let save_data = format!("{} {}", msg.channel_id, msg.id);
    match file.write_all(save_data.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            msg.reply(ctx, "Something went wrong").await?;
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
                Err(e) => eprintln!("Failed to wait for command: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
    msg.reply(ctx, "something went wrong, check the logs or try again")
        .await?;
    Ok(())
}

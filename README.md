# Rust Bot

Rust Bot is an open-source general-purpose Discord bot written in Rust. It is designed to be highly extensible, allowing you to add your own custom commands and features. This README provides an overview of the project, its structure, and how to get started with running and extending the bot.

## Table of Contents

- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Adding Custom Commands](#adding-custom-commands)
- [Contributing](#contributing)
- [License](#license)

## Project Structure

The Rust Bot project is organized into several modules and files, each serving a specific purpose. Here's a brief overview of the project structure:

### Main Entry Point
- [main.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/main.rs): This is the main entry point for the bot. It sets up the bot, handles events, and starts the bot. It also handles bot restarts triggered by updates.

### Commands
- [commands/](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/): This directory contains all the bot's commands, each in its own module file. Here are some notable commands:
    - [play.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/play.rs): Plays a song from a YouTube URL in a voice channel.
    - [test.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/test.rs): A test command for various bot functionalities.
    - [ustatus.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/ustatus.rs): Checks the status of a website (e.g., "uninotes.mantarias.com").
    - [pfp.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/pfp.rs): Returns the profile picture of a mentioned user.
    - [coinflip.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/coinflip.rs): Simulates a coin flip and announces the result.
    - [update.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/update.rs): Updates the bot by pulling from GitHub and restarting its service.
    - [stats.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/stats.rs): Collects message statistics from the channel and generates a pie chart.
    - [uwu.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/uwu.rs): Converts text to "uwu" language with random emoticons.
    - [poll.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/poll.rs): Creates a poll with reactions for users to vote.

### Command Module
- [commands/mod.rs](/Users/lennartdiegokahn/Documents/rust_bot/src/commands/mod.rs): This file loads all the commands in the `commands` directory and defines the `General` command group.

### Event Handler
- [Handler](/Users/lennartdiegokahn/Documents/rust_bot/src/main.rs#L34-L40): This struct implements the `EventHandler` trait, handling Discord events. You can extend this to add custom event handling logic.

### Dependencies and Configuration
- [Cargo.toml](/Users/lennartdiegokahn/Documents/rust_bot/Cargo.toml): The project's dependencies and configuration, including the Discord token and other environment variables.

## Getting Started

To run Rust Bot, follow these steps:

1. Clone the repository to your local machine.

2. Ensure you have Rust and Cargo installed. You can download them from the [official website](https://www.rust-lang.org/learn/get-started).

3. Create a `.env` file in the project's root directory and add your Discord bot token like this:
   ```
   DISCORD_BOT_TOKEN=your_token_here
   ```

4. Build and run the bot using the following command:
   ```
   cargo run
   ```

5. Once the bot is running, invite it to your Discord server and start using the available commands.

## Adding Custom Commands

Rust Bot is designed to be extensible, and you can easily add your own custom commands. Here's how to do it:

1. Create a new module file for your command in the `commands` directory. You can follow the structure of existing command modules.

2. Define your custom command using the `#[command]` attribute. Implement the command logic as an asynchronous function.

3. Add your command module to the `commands/mod.rs` file by importing it and adding it to the `#[group]` attribute.

4. Build and run the bot to test your custom command.

5. You can now use your custom command in Discord by prefixing it with the bot's command prefix.

## Contributing

If you'd like to contribute to Rust Bot, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bug fix.

2. Implement your changes and make sure they follow Rust's coding conventions.

3. Write tests for your code if applicable.

4. Create a pull request with a clear description of your changes and why they are necessary.

5. Your pull request will be reviewed, and once approved, it will be merged into the main branch.

6. Thank you for contributing to Rust Bot!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute the code for your own purposes.
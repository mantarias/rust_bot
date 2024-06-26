//! # mod.rs
//!
//! This file is used to load all the commands in the commands folder.
//! The commands are:
//! * echo - Echoes the input message back to the user
//! * ping - Pings the bot and returns the round trip time
//! * stats - Returns stats of the last x amount of messages and how many times a user has sent a message
//! * status - Returns the status of a website
//! * test - Used to test new the bot commands not yet ready for production
//! * update - Updates the bot by pulling from github and restarting its service
//! * ustatus - Checks the status of uninotes
//! * pfp - Returns the profile picture of a pinged user
//! * uwu - uwunizes a message
//! * play - Plays a song from youtube
//! * poll - Creates a poll
//! * coinflip - Flips a coin

pub mod coinflip;
pub mod echo;
pub mod pfp;
pub mod ping;
pub mod play;
pub mod poll;
mod react;
pub mod stats;
pub mod status;
pub mod test;
pub mod update;
pub mod uwu;
pub mod idk;

use self::{
    coinflip::*, echo::*, pfp::*, ping::*, play::*, poll::*, stats::*, status::*, test::*,
    update::*, uwu::*, idk::*,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(
    ping, test, echo, status, update, status, stats, pfp, uwu, play, poll, coinflip, idk
)] // Add other commands here
struct General;

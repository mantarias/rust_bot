pub mod echo;
pub mod ping;
pub mod stats;
pub mod status;
pub mod test;
pub mod update;
pub mod ustatus;
pub mod pfp;
pub mod uwu;
pub mod play;
pub mod poll;
pub mod coinflip;


use self::{
    echo::*,
    ping::*,
    stats::*,
    status::*,
    test::*,
    update::*,
    ustatus::*,
    pfp::*,
    uwu::*,
    play::*,
    poll::*,
    coinflip::*,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, test, echo, status, update, status, ustatus, stats, pfp, uwu, play, poll, coinflip)] // Add other commands here
struct General;

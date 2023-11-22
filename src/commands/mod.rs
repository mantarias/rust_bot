pub mod echo;
pub mod ping;
pub mod stats;
pub mod status;
pub mod test;
pub mod update;
pub mod ustatus;
pub mod pfp;

use self::{
    echo::*,
    ping::*,
    stats::*,
    status::*,
    test::*,
    update::*,
    ustatus::*,
    pfp::*,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, test, echo, status, update, status, ustatus, stats, pfp)] // Add other commands here
struct General;

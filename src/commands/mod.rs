pub mod echo;
pub mod ping;
pub mod stats;
pub mod status;
pub mod test;
pub mod update;
pub mod ustatus;

use self::{
    echo::*,
    ping::*,
    stats::*,
    status::*,
    test::*,
    update::*,
    ustatus::*,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, test, echo, status, update, status, ustatus, stats)] // Add other commands here
struct General;

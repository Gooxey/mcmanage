//! This module implements the [`SetId`](Command::SetId) command.


use proc_macros::add_convert;

use super::Command;


/// These are the arguments for the [`SetId`](Command::SetId) command.
#[add_convert]
pub struct SetIdArgs {
    /// The id to be set
    pub id: u64
}

impl Command {
    /// Execute the [`SetId`](Command::SetId) command.
    pub async fn execute_setid(self, _args: SetIdArgs) {

    }
}
//! This module implements the [`Error`](Command::Error) command.

use proc_macros::add_convert;

use super::Command;
use crate::mcmanage_error::MCManageError;

/// These are the arguments for the [`Error`](Command::Error) command.
#[add_convert]
pub struct ErrorArgs {
    /// The error in string form
    pub error: String,
}

impl Command {
    /// Execute the [`Error`](Command::Error) command.
    pub async fn execute_error(self, _args: ErrorArgs) {}
}

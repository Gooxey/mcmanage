//! This module provides various types which make it easier to read the code

use tokio::task::JoinHandle;

use crate::mcmanage_error::MCManageError;

/// This type describes the JoinHandle for threads of struct implementing [`ConcurrentClass`](proc_macros::ConcurrentClass)
pub type ThreadJoinHandle = JoinHandle<Result<(), MCManageError>>;

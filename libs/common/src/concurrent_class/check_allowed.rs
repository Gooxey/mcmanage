use tokio::{sync::Mutex, time::sleep};
use goolog::*;

use crate::{status::Status, mcmanage_error::MCManageError, config::Config};

/// Check if a start method is allowed to be executed. \
/// This function will also set the given status to the right value.
///
/// # Returns
///
/// | Return                                | Description                                               |
/// |---------------------------------------|-----------------------------------------------------------|
/// | `Ok(())`                              | The method can be executed immediately.                   |
/// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
/// | [`MCManageError::NotReady`]           | The method can not be used.                               |
pub async fn check_allowed_start(status: &Mutex<Status>, restart: bool) -> Result<(), MCManageError> {
    let mut status_lock = status.lock().await;
    match *status_lock {
        Status::Started => return Err(MCManageError::AlreadyExecuted),
        Status::Starting => return Err(MCManageError::CurrentlyExecuting),
        Status::Stopped => {
            if !restart {
                *status_lock = Status::Starting;
            }
            return Ok(())
        },
        Status::Stopping => return Err(MCManageError::NotReady),
        Status::Restarting => {
            if !restart {
                return Err(MCManageError::CurrentlyExecuting)
            } else {
                return Ok(())
            }
        }
    }
}
/// Check if stop method is allowed to be executed. \
/// This function will also set the given status to the right value. \
/// If the `forced` parameter got set to true this function will wait until the class has either started or stopped.
///
/// # Returns
///
/// | Return                                | Description                                               |
/// |---------------------------------------|-----------------------------------------------------------|
/// | `Ok(())`                              | The method can be executed immediately.                   |
/// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
/// | [`MCManageError::NotReady`]           | The method can not be used.                               |
pub async fn check_allowed_stop(status: &Mutex<Status>, restart: bool, forced: bool, caller: &str) -> Result<(), MCManageError> {
    if forced && !restart {
        info!(caller; "Waiting to be fully started before stopping...");
        // wait till the class has started
        loop {
            if let Status::Started = *status.lock().await {
                break;
            }
            sleep(Config::cooldown().await).await;
        }
    }

    let mut status_lock = status.lock().await;
    match *status_lock {
        Status::Started => {
            if !restart {
                *status_lock = Status::Stopping;
            }
            return Ok(())
        }
        Status::Starting => return Err(MCManageError::NotReady),
        Status::Stopped => return Err(MCManageError::AlreadyExecuted),
        Status::Stopping => return Err(MCManageError::CurrentlyExecuting),
        Status::Restarting => {
            if !restart {
                return Err(MCManageError::NotReady)
            } else {
                return Ok(())
            }
        }
    }
}
/// Check if a restart method is allowed to be executed. \
/// This function will also set the given status to the right value.
///
/// # Returns
///
/// | Return                                | Description                                                               |
/// |---------------------------------------|---------------------------------------------------------------------------|
/// | `Ok(())`                              | The method can be executed immediately.                                   |
/// | [`MCManageError::NotStarted`]         | The method can not be executed since the given struct is not yet started. |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread.                 |
/// | [`MCManageError::NotReady`]           | The method can not be used.                                               |
pub async fn check_allowed_restart(status: &Mutex<Status>) -> Result<(), MCManageError> {
    let mut status_lock = status.lock().await;
    match *status_lock {
        Status::Started => {
            *status_lock = Status::Restarting;
            return Ok(())
        }
        Status::Starting => return Err(MCManageError::NotReady),
        Status::Stopped => return Err(MCManageError::NotStarted),
        Status::Stopping => return Err(MCManageError::NotStarted),
        Status::Restarting => return Err(MCManageError::CurrentlyExecuting),
    }
}
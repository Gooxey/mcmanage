//! This module provides the [`Status enum`](Status), which gets used for the representation of the status of structs implementing the [`ConcurrentClass`](proc_macros::ConcurrentClass) proc macro.

/// This enum represents the status of structs implementing the [`ConcurrentClass`](proc_macros::ConcurrentClass) proc macro.
///
/// # Variant
///
/// | Variant                            | Description                                                                                                                   |
/// |------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
/// | [`Stopped`](Status::Stopped)       | The struct is currently inactive. Therefore, any operation other than the start method is not possible to use.                |
/// | [`Started`](Status::Started)       | The struct is currently inactive. Therefore, any operation other than the start method is possible to use.                    |
/// | [`Starting`](Status::Starting)     | The struct is currently starting. It will be fully functional as soon as the status switches to [`Started`](Status::Started). |
/// | [`Stopping`](Status::Stopping)     | The struct is currently stopping. Before doing anything, wait for the status to change to [`Stopped`](Status::Stopped).       |
/// | [`Restarting`](Status::Restarting) | The struct is currently restarting. Wait for the status to change to [`Started`](Status::Started) for full functionality.     |
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Status {
    /// The struct is currently inactive. Therefore, any operation other than the start method is not possible to use.
    Stopped,
    /// The struct is currently inactive. Therefore, any operation other than the start method is possible to use.
    Started,
    /// The struct is currently starting. It will be fully functional as soon as the status switches to [`Started`](Status::Started).
    Starting,
    /// The struct is currently stopping. Before doing anything, wait for the status to change to [`Stopped`](Status::Stopped).
    Stopping,
    /// The struct is currently restarting. Wait for the status to change to [`Started`](Status::Started) for full functionality.
    Restarting,
}

//! This module provides the [`Permission`] enum which describes who is allowed to execute the [`Command`](super::Command).

/// This enum describes who is allowed to execute this [`Command`](super::Command).
pub enum Permission {
    /// Everyone has the rights to execute this [`Command`](super::Command).
    All,
    /// Only the main application can execute this [`Command`](super::Command)
    Main,
    /// Only the clients of type [`Worker`](super::super::super::client_type::ClientType::Worker) are allowed to execute this [`Command`](super::Command).
    Worker,
}

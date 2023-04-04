//! This module provides the [`ReservedConnections`] struct which contains all connections to every worker registered.

use self::connection::Connection;

pub mod connection;

/// This struct contains all connections to every worker registered.
pub struct ReservedConnections {
    /// This is a list of all registered [`Connection`]s to workers
    connections: Vec<Connection>,
}
impl ReservedConnections {
    /// Create a new [`ReservedConnections`] instance.
    pub fn new() -> Self {
        Self {
            connections: vec![],
        }
    }
    /// Return how many workers are registered.
    pub fn count(&self) -> usize {
        self.connections.len()
    }
}

use std::sync::{atomic::{AtomicI64, Ordering}, Arc};

use goodmetrics::SumHandle;

pub struct ConnectionGuard {
    connections_closed: SumHandle,
    total_active_connections: SumHandle,
    active_connections_counter: Arc<AtomicI64>,
}

impl ConnectionGuard {
    /// Creates a new `ConnectionGuard` instance and increments the `total_active_connections` counter.
    /// Decrements the `total_active_connections` counter when the `ConnectionGuard` is dropped.
    pub fn new(
        connections_opened: SumHandle,
        connections_closed: SumHandle,
        total_active_connections: SumHandle,
        active_connections_counter: Arc<AtomicI64>,
    ) -> Self {
        connections_opened.observe(1);
        total_active_connections.observe(1);
        Self {
            connections_closed,
            total_active_connections,
            active_connections_counter,
        }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // When the guard is dropped, we assume the connection is closed.
        self.connections_closed.observe(1);
        self.total_active_connections.observe(-1);
    }
}

use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

use goodmetrics::SumHandle;

/// Guard that records connection open/close counts; records closure when dropped.
pub struct ConnectionGuard {
    connections_closed: SumHandle,
    total_active_connections_count: Arc<AtomicI64>,
}

impl ConnectionGuard {
    /// Creates a guard, incrementing the opened counter and active connection count.
    pub fn new(
        connections_opened: SumHandle,
        connections_closed: SumHandle,
        total_active_connections_count: Arc<AtomicI64>,
    ) -> Self {
        connections_opened.observe(1);
        total_active_connections_count.fetch_add(1, Ordering::Relaxed);
        Self {
            connections_closed,
            total_active_connections_count,
        }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // When the guard is dropped, we assume the connection is closed.
        self.connections_closed.observe(1);
        self.total_active_connections_count
            .fetch_sub(1, Ordering::Relaxed);
    }
}

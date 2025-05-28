use goodmetrics::SumHandle;

pub struct ConnectionGuard {
    connections_closed: SumHandle,
    total_active_connections: SumHandle,
}

impl ConnectionGuard {
    pub fn new(
        connections_opened: SumHandle,
        connections_closed: SumHandle,
        total_active_connections: SumHandle,
    ) -> Self {
        connections_opened.observe(1);
        total_active_connections.observe(1);
        Self {
            connections_closed,
            total_active_connections,
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

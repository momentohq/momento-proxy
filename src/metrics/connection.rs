use goodmetrics::SumHandle;

pub struct ConnectionGuard {
    connections_closed: SumHandle,
}

impl ConnectionGuard {
    /// Creates a new `ConnectionGuard` instance.
    /// This instance will increment the `connections_opened` counter
    pub fn new(connections_opened: SumHandle, connections_closed: SumHandle) -> Self {
        connections_opened.observe(1);
        Self { connections_closed }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // When the guard is dropped, we assume the connection is closed.
        self.connections_closed.observe(1);
    }
}

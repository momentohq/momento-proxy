use goodmetrics::SumHandle;

pub struct ConnectionGuard {
    connections_closed: SumHandle,
    decrement_total_count_fn: Box<dyn Fn() + Send>,
}

impl ConnectionGuard {
    /// Creates a new `ConnectionGuard` instance and increments the `total_active_connections` counter.
    /// Decrements the `total_active_connections` counter when the `ConnectionGuard` is dropped.
    pub fn new(
        connections_opened: SumHandle,
        connections_closed: SumHandle,
        decrement_total_count_fn: impl Fn() + 'static + Send,
    ) -> Self {
        connections_opened.observe(1);
        Self {
            connections_closed,
            decrement_total_count_fn: Box::new(decrement_total_count_fn),
        }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // When the guard is dropped, we assume the connection is closed.
        self.connections_closed.observe(1);
        (self.decrement_total_count_fn)();
    }
}

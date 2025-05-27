use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

use super::ConnectionGuard;
use goodmetrics::SumHandle;

use super::{RpcCallGuard, RpcMetrics};

pub trait ProxyMetrics: Clone + Send + Sync + 'static {
    fn begin_connection(&self) -> ConnectionGuard;
    fn begin_memcached_get(&self) -> RpcCallGuard;
    fn begin_memcached_set(&self) -> RpcCallGuard;
    fn begin_memcached_delete(&self) -> RpcCallGuard;
    fn begin_memcached_unimplemented(&self) -> RpcCallGuard;
}

#[derive(Clone, Debug)]
pub struct DefaultProxyMetrics {
    pub(crate) memcached_get: RpcMetrics,
    pub(crate) memcached_set: RpcMetrics,
    pub(crate) memcached_delete: RpcMetrics,
    pub(crate) memcached_unimplemented: RpcMetrics,
    pub(crate) connections_opened: SumHandle,
    pub(crate) connections_closed: SumHandle,
    pub(crate) total_active_connections: SumHandle,
    pub(crate) active_connections_counter: Arc<AtomicI64>,
}

impl ProxyMetrics for DefaultProxyMetrics {
    fn begin_connection(&self) -> ConnectionGuard {
        let count = self
            .active_connections_counter
            .fetch_add(1, Ordering::Relaxed);
        self.total_active_connections.observe(count as i64);
        debug!("Incrementing active connections: {}", count+1);
        let total_active_connections = self.total_active_connections.clone();
        let active_connections_counter = self.active_connections_counter.clone();
        ConnectionGuard::new(
            self.connections_opened.clone(),
            self.connections_closed.clone(),
            move || {
                let count = active_connections_counter.fetch_sub(1, Ordering::Relaxed);
                total_active_connections.observe(count as i64);
                debug!("Decrementing active connections: {}", count+1);
            },
        )
    }

    fn begin_memcached_get(&self) -> RpcCallGuard {
        self.memcached_get.record_api_call()
    }

    fn begin_memcached_set(&self) -> RpcCallGuard {
        self.memcached_set.record_api_call()
    }

    fn begin_memcached_delete(&self) -> RpcCallGuard {
        self.memcached_delete.record_api_call()
    }

    fn begin_memcached_unimplemented(&self) -> RpcCallGuard {
        self.memcached_unimplemented.record_api_call()
    }
}

impl ProxyMetrics for Arc<DefaultProxyMetrics> {
    fn begin_connection(&self) -> ConnectionGuard {
        self.as_ref().begin_connection()
    }

    fn begin_memcached_get(&self) -> RpcCallGuard {
        self.as_ref().begin_memcached_get()
    }

    fn begin_memcached_set(&self) -> RpcCallGuard {
        self.as_ref().begin_memcached_set()
    }

    fn begin_memcached_delete(&self) -> RpcCallGuard {
        self.as_ref().begin_memcached_delete()
    }

    fn begin_memcached_unimplemented(&self) -> RpcCallGuard {
        self.as_ref().begin_memcached_unimplemented()
    }
}

use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};

use super::{
    util::{proxy_statistic_set_gauge, proxy_sum_gauge},
    ConnectionGuard,
};
use goodmetrics::{GaugeFactory, SumHandle};

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
    pub(crate) total_active_connections_count: Arc<AtomicI64>,
}

impl DefaultProxyMetrics {
    pub(crate) fn new(gauge_factory: &GaugeFactory, batch_interval: Duration) -> Self {
        // Keep track of the total number of active connections to the proxy
        let total_active_connections_count = Arc::new(AtomicI64::new(0));
        let total_active_connections =
            proxy_statistic_set_gauge(gauge_factory, "total_active_connections");

        // Emit the total active connections count every batch_interval seconds
        let count_clone = total_active_connections_count.clone();
        tokio::spawn(async move {
            loop {
                total_active_connections.observe(count_clone.load(Ordering::Relaxed));
                tokio::time::sleep(batch_interval).await;
            }
        });

        // Create the remaining gauge handles
        Self {
            memcached_get: RpcMetrics::new(gauge_factory, "memcached_get"),
            memcached_set: RpcMetrics::new(gauge_factory, "memcached_set"),
            memcached_delete: RpcMetrics::new(gauge_factory, "memcached_delete"),
            memcached_unimplemented: RpcMetrics::new(gauge_factory, "memcached_unimplemented"),
            connections_opened: proxy_sum_gauge(gauge_factory, "connections_opened"),
            connections_closed: proxy_sum_gauge(gauge_factory, "connections_closed"),
            total_active_connections_count,
        }
    }
}

impl ProxyMetrics for DefaultProxyMetrics {
    fn begin_connection(&self) -> ConnectionGuard {
        ConnectionGuard::new(
            self.connections_opened.clone(),
            self.connections_closed.clone(),
            self.total_active_connections_count.clone(),
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

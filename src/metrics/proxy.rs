use std::sync::Arc;

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
}

impl ProxyMetrics for DefaultProxyMetrics {
    fn begin_connection(&self) -> ConnectionGuard {
        ConnectionGuard::new(
            self.connections_opened.clone(),
            self.connections_closed.clone(),
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

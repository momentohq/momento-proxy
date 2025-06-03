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

pub trait ConnectionMetrics: Clone + Send + Sync + 'static {
    fn begin_connection(&self) -> ConnectionGuard;
}

pub trait MemcachedMetrics: Clone + Send + Sync + 'static {
    fn begin_memcached_get(&self) -> RpcCallGuard;
    fn begin_memcached_set(&self) -> RpcCallGuard;
    fn begin_memcached_delete(&self) -> RpcCallGuard;
    fn begin_memcached_unimplemented(&self) -> RpcCallGuard;
}

pub trait RespMetrics: Clone + Send + Sync + 'static {
    fn begin_resp_del(&self) -> RpcCallGuard;
    fn begin_resp_get(&self) -> RpcCallGuard;
    fn begin_resp_hdel(&self) -> RpcCallGuard;
    fn begin_resp_hexists(&self) -> RpcCallGuard;
    fn begin_resp_hget(&self) -> RpcCallGuard;
    fn begin_resp_hgetall(&self) -> RpcCallGuard;
    fn begin_resp_hincrby(&self) -> RpcCallGuard;
    fn begin_resp_hkeys(&self) -> RpcCallGuard;
    fn begin_resp_hlen(&self) -> RpcCallGuard;
    fn begin_resp_hmget(&self) -> RpcCallGuard;
    fn begin_resp_hset(&self) -> RpcCallGuard;
    fn begin_resp_hvals(&self) -> RpcCallGuard;
    fn begin_resp_lindex(&self) -> RpcCallGuard;
    fn begin_resp_llen(&self) -> RpcCallGuard;
    fn begin_resp_lpop(&self) -> RpcCallGuard;
    fn begin_resp_lrange(&self) -> RpcCallGuard;
    fn begin_resp_lpush(&self) -> RpcCallGuard;
    fn begin_resp_rpush(&self) -> RpcCallGuard;
    fn begin_resp_rpop(&self) -> RpcCallGuard;
    fn begin_resp_set(&self) -> RpcCallGuard;
    fn begin_resp_sadd(&self) -> RpcCallGuard;
    fn begin_resp_srem(&self) -> RpcCallGuard;
    fn begin_resp_sdiff(&self) -> RpcCallGuard;
    fn begin_resp_sunion(&self) -> RpcCallGuard;
    fn begin_resp_sinter(&self) -> RpcCallGuard;
    fn begin_resp_smembers(&self) -> RpcCallGuard;
    fn begin_resp_sismember(&self) -> RpcCallGuard;
    fn begin_resp_zcard(&self) -> RpcCallGuard;
    fn begin_resp_zincrby(&self) -> RpcCallGuard;
    fn begin_resp_zscore(&self) -> RpcCallGuard;
    fn begin_resp_zmscore(&self) -> RpcCallGuard;
    fn begin_resp_zrem(&self) -> RpcCallGuard;
    fn begin_resp_zrank(&self) -> RpcCallGuard;
    fn begin_resp_zrange(&self) -> RpcCallGuard;
    fn begin_resp_zadd(&self) -> RpcCallGuard;
    fn begin_resp_zrevrank(&self) -> RpcCallGuard;
    fn begin_resp_zcount(&self) -> RpcCallGuard;
    fn begin_resp_zunionstore(&self) -> RpcCallGuard;
    fn begin_resp_unimplemented(&self) -> RpcCallGuard;
}

pub trait ProxyMetrics: ConnectionMetrics + MemcachedMetrics + RespMetrics {}
impl<T: ConnectionMetrics + MemcachedMetrics + RespMetrics> ProxyMetrics for T {}

#[derive(Clone, Debug)]
pub struct DefaultProxyMetrics {
    // connection handles
    pub(crate) connections_opened: SumHandle,
    pub(crate) connections_closed: SumHandle,
    pub(crate) total_active_connections_count: Arc<AtomicI64>,

    // memcached handles
    pub(crate) memcached_get: RpcMetrics,
    pub(crate) memcached_set: RpcMetrics,
    pub(crate) memcached_delete: RpcMetrics,
    pub(crate) memcached_unimplemented: RpcMetrics,

    // resp handles
    pub(crate) resp_del: RpcMetrics,
    pub(crate) resp_get: RpcMetrics,
    pub(crate) resp_hdel: RpcMetrics,
    pub(crate) resp_hexists: RpcMetrics,
    pub(crate) resp_hget: RpcMetrics,
    pub(crate) resp_hgetall: RpcMetrics,
    pub(crate) resp_hincrby: RpcMetrics,
    pub(crate) resp_hkeys: RpcMetrics,
    pub(crate) resp_hlen: RpcMetrics,
    pub(crate) resp_hmget: RpcMetrics,
    pub(crate) resp_hset: RpcMetrics,
    pub(crate) resp_hvals: RpcMetrics,
    pub(crate) resp_lindex: RpcMetrics,
    pub(crate) resp_llen: RpcMetrics,
    pub(crate) resp_lpop: RpcMetrics,
    pub(crate) resp_lrange: RpcMetrics,
    pub(crate) resp_lpush: RpcMetrics,
    pub(crate) resp_rpush: RpcMetrics,
    pub(crate) resp_rpop: RpcMetrics,
    pub(crate) resp_set: RpcMetrics,
    pub(crate) resp_sadd: RpcMetrics,
    pub(crate) resp_srem: RpcMetrics,
    pub(crate) resp_sdiff: RpcMetrics,
    pub(crate) resp_sunion: RpcMetrics,
    pub(crate) resp_sinter: RpcMetrics,
    pub(crate) resp_smembers: RpcMetrics,
    pub(crate) resp_sismember: RpcMetrics,
    pub(crate) resp_zcard: RpcMetrics,
    pub(crate) resp_zincrby: RpcMetrics,
    pub(crate) resp_zscore: RpcMetrics,
    pub(crate) resp_zmscore: RpcMetrics,
    pub(crate) resp_zrem: RpcMetrics,
    pub(crate) resp_zrank: RpcMetrics,
    pub(crate) resp_zrange: RpcMetrics,
    pub(crate) resp_zadd: RpcMetrics,
    pub(crate) resp_zrevrank: RpcMetrics,
    pub(crate) resp_zcount: RpcMetrics,
    pub(crate) resp_zunionstore: RpcMetrics,
    pub(crate) resp_unimplemented: RpcMetrics,
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
            resp_del: RpcMetrics::new(gauge_factory, "resp_del"),
            resp_get: RpcMetrics::new(gauge_factory, "resp_get"),
            resp_hdel: RpcMetrics::new(gauge_factory, "resp_hdel"),
            resp_hexists: RpcMetrics::new(gauge_factory, "resp_hexists"),
            resp_hget: RpcMetrics::new(gauge_factory, "resp_hget"),
            resp_hgetall: RpcMetrics::new(gauge_factory, "resp_hgetall"),
            resp_hincrby: RpcMetrics::new(gauge_factory, "resp_hincrby"),
            resp_hkeys: RpcMetrics::new(gauge_factory, "resp_hkeys"),
            resp_hlen: RpcMetrics::new(gauge_factory, "resp_hlen"),
            resp_hmget: RpcMetrics::new(gauge_factory, "resp_hmget"),
            resp_hset: RpcMetrics::new(gauge_factory, "resp_hset"),
            resp_hvals: RpcMetrics::new(gauge_factory, "resp_hvals"),
            resp_lindex: RpcMetrics::new(gauge_factory, "resp_lindex"),
            resp_llen: RpcMetrics::new(gauge_factory, "resp_llen"),
            resp_lpop: RpcMetrics::new(gauge_factory, "resp_lpop"),
            resp_lrange: RpcMetrics::new(gauge_factory, "resp_lrange"),
            resp_lpush: RpcMetrics::new(gauge_factory, "resp_lpush"),
            resp_rpush: RpcMetrics::new(gauge_factory, "resp_rpush"),
            resp_rpop: RpcMetrics::new(gauge_factory, "resp_rpop"),
            resp_set: RpcMetrics::new(gauge_factory, "resp_set"),
            resp_sadd: RpcMetrics::new(gauge_factory, "resp_sadd"),
            resp_srem: RpcMetrics::new(gauge_factory, "resp_srem"),
            resp_sdiff: RpcMetrics::new(gauge_factory, "resp_sdiff"),
            resp_sunion: RpcMetrics::new(gauge_factory, "resp_sunion"),
            resp_sinter: RpcMetrics::new(gauge_factory, "resp_sinter"),
            resp_smembers: RpcMetrics::new(gauge_factory, "resp_smembers"),
            resp_sismember: RpcMetrics::new(gauge_factory, "resp_sismember"),
            resp_zcard: RpcMetrics::new(gauge_factory, "resp_zcard"),
            resp_zincrby: RpcMetrics::new(gauge_factory, "resp_zincrby"),
            resp_zscore: RpcMetrics::new(gauge_factory, "resp_zscore"),
            resp_zmscore: RpcMetrics::new(gauge_factory, "resp_zmscore"),
            resp_zrem: RpcMetrics::new(gauge_factory, "resp_zrem"),
            resp_zrank: RpcMetrics::new(gauge_factory, "resp_zrank"),
            resp_zrange: RpcMetrics::new(gauge_factory, "resp_zrange"),
            resp_zadd: RpcMetrics::new(gauge_factory, "resp_zadd"),
            resp_zrevrank: RpcMetrics::new(gauge_factory, "resp_zrevrank"),
            resp_zcount: RpcMetrics::new(gauge_factory, "resp_zcount"),
            resp_zunionstore: RpcMetrics::new(gauge_factory, "resp_zunionstore"),
            resp_unimplemented: RpcMetrics::new(gauge_factory, "resp_unimplemented"),
            connections_opened: proxy_sum_gauge(gauge_factory, "connections_opened"),
            connections_closed: proxy_sum_gauge(gauge_factory, "connections_closed"),
            total_active_connections_count,
        }
    }
}

impl ConnectionMetrics for DefaultProxyMetrics {
    fn begin_connection(&self) -> ConnectionGuard {
        ConnectionGuard::new(
            self.connections_opened.clone(),
            self.connections_closed.clone(),
            self.total_active_connections_count.clone(),
        )
    }
}

impl MemcachedMetrics for DefaultProxyMetrics {
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

impl RespMetrics for DefaultProxyMetrics {
    fn begin_resp_del(&self) -> RpcCallGuard {
        self.resp_del.record_api_call()
    }
    fn begin_resp_get(&self) -> RpcCallGuard {
        self.resp_get.record_api_call()
    }
    fn begin_resp_hdel(&self) -> RpcCallGuard {
        self.resp_hdel.record_api_call()
    }
    fn begin_resp_hexists(&self) -> RpcCallGuard {
        self.resp_hexists.record_api_call()
    }
    fn begin_resp_hget(&self) -> RpcCallGuard {
        self.resp_hget.record_api_call()
    }
    fn begin_resp_hgetall(&self) -> RpcCallGuard {
        self.resp_hgetall.record_api_call()
    }
    fn begin_resp_hincrby(&self) -> RpcCallGuard {
        self.resp_hincrby.record_api_call()
    }
    fn begin_resp_hkeys(&self) -> RpcCallGuard {
        self.resp_hkeys.record_api_call()
    }
    fn begin_resp_hlen(&self) -> RpcCallGuard {
        self.resp_hlen.record_api_call()
    }
    fn begin_resp_hmget(&self) -> RpcCallGuard {
        self.resp_hmget.record_api_call()
    }
    fn begin_resp_hset(&self) -> RpcCallGuard {
        self.resp_hset.record_api_call()
    }
    fn begin_resp_hvals(&self) -> RpcCallGuard {
        self.resp_hvals.record_api_call()
    }
    fn begin_resp_lindex(&self) -> RpcCallGuard {
        self.resp_lindex.record_api_call()
    }
    fn begin_resp_llen(&self) -> RpcCallGuard {
        self.resp_llen.record_api_call()
    }
    fn begin_resp_lpop(&self) -> RpcCallGuard {
        self.resp_lpop.record_api_call()
    }
    fn begin_resp_lrange(&self) -> RpcCallGuard {
        self.resp_lrange.record_api_call()
    }
    fn begin_resp_lpush(&self) -> RpcCallGuard {
        self.resp_lpush.record_api_call()
    }
    fn begin_resp_rpush(&self) -> RpcCallGuard {
        self.resp_rpush.record_api_call()
    }
    fn begin_resp_rpop(&self) -> RpcCallGuard {
        self.resp_rpop.record_api_call()
    }
    fn begin_resp_set(&self) -> RpcCallGuard {
        self.resp_set.record_api_call()
    }
    fn begin_resp_sadd(&self) -> RpcCallGuard {
        self.resp_sadd.record_api_call()
    }
    fn begin_resp_srem(&self) -> RpcCallGuard {
        self.resp_srem.record_api_call()
    }
    fn begin_resp_sdiff(&self) -> RpcCallGuard {
        self.resp_sdiff.record_api_call()
    }
    fn begin_resp_sunion(&self) -> RpcCallGuard {
        self.resp_sunion.record_api_call()
    }
    fn begin_resp_sinter(&self) -> RpcCallGuard {
        self.resp_sinter.record_api_call()
    }
    fn begin_resp_smembers(&self) -> RpcCallGuard {
        self.resp_smembers.record_api_call()
    }
    fn begin_resp_sismember(&self) -> RpcCallGuard {
        self.resp_sismember.record_api_call()
    }
    fn begin_resp_zcard(&self) -> RpcCallGuard {
        self.resp_zcard.record_api_call()
    }
    fn begin_resp_zincrby(&self) -> RpcCallGuard {
        self.resp_zincrby.record_api_call()
    }
    fn begin_resp_zscore(&self) -> RpcCallGuard {
        self.resp_zscore.record_api_call()
    }
    fn begin_resp_zmscore(&self) -> RpcCallGuard {
        self.resp_zmscore.record_api_call()
    }
    fn begin_resp_zrem(&self) -> RpcCallGuard {
        self.resp_zrem.record_api_call()
    }
    fn begin_resp_zrank(&self) -> RpcCallGuard {
        self.resp_zrank.record_api_call()
    }
    fn begin_resp_zrange(&self) -> RpcCallGuard {
        self.resp_zrange.record_api_call()
    }
    fn begin_resp_zadd(&self) -> RpcCallGuard {
        self.resp_zadd.record_api_call()
    }
    fn begin_resp_zrevrank(&self) -> RpcCallGuard {
        self.resp_zrevrank.record_api_call()
    }
    fn begin_resp_zcount(&self) -> RpcCallGuard {
        self.resp_zcount.record_api_call()
    }
    fn begin_resp_zunionstore(&self) -> RpcCallGuard {
        self.resp_zunionstore.record_api_call()
    }
    fn begin_resp_unimplemented(&self) -> RpcCallGuard {
        self.resp_unimplemented.record_api_call()
    }
}

impl ConnectionMetrics for Arc<DefaultProxyMetrics> {
    fn begin_connection(&self) -> ConnectionGuard {
        self.as_ref().begin_connection()
    }
}

impl MemcachedMetrics for Arc<DefaultProxyMetrics> {
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

impl RespMetrics for Arc<DefaultProxyMetrics> {
    fn begin_resp_del(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_del()
    }
    fn begin_resp_get(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_get()
    }
    fn begin_resp_hdel(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hdel()
    }
    fn begin_resp_hexists(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hexists()
    }
    fn begin_resp_hget(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hget()
    }
    fn begin_resp_hgetall(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hgetall()
    }
    fn begin_resp_hincrby(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hincrby()
    }
    fn begin_resp_hkeys(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hkeys()
    }
    fn begin_resp_hlen(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hlen()
    }
    fn begin_resp_hmget(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hmget()
    }
    fn begin_resp_hset(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hset()
    }
    fn begin_resp_hvals(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_hvals()
    }
    fn begin_resp_lindex(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_lindex()
    }
    fn begin_resp_llen(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_llen()
    }
    fn begin_resp_lpop(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_lpop()
    }
    fn begin_resp_lrange(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_lrange()
    }
    fn begin_resp_lpush(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_lpush()
    }
    fn begin_resp_rpush(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_rpush()
    }
    fn begin_resp_rpop(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_rpop()
    }
    fn begin_resp_set(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_set()
    }
    fn begin_resp_sadd(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_sadd()
    }
    fn begin_resp_srem(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_srem()
    }
    fn begin_resp_sdiff(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_sdiff()
    }
    fn begin_resp_sunion(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_sunion()
    }
    fn begin_resp_sinter(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_sinter()
    }
    fn begin_resp_smembers(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_smembers()
    }
    fn begin_resp_sismember(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_sismember()
    }
    fn begin_resp_zcard(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zcard()
    }
    fn begin_resp_zincrby(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zincrby()
    }
    fn begin_resp_zscore(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zscore()
    }
    fn begin_resp_zmscore(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zmscore()
    }
    fn begin_resp_zrem(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zrem()
    }
    fn begin_resp_zrank(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zrank()
    }
    fn begin_resp_zrange(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zrange()
    }
    fn begin_resp_zadd(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zadd()
    }
    fn begin_resp_zrevrank(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zrevrank()
    }
    fn begin_resp_zcount(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zcount()
    }
    fn begin_resp_zunionstore(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_zunionstore()
    }
    fn begin_resp_unimplemented(&self) -> RpcCallGuard {
        self.as_ref().begin_resp_unimplemented()
    }
}

use std::{
    borrow::Borrow,
    hash::RandomState,
    mem::size_of,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use k_cache::SegmentedCache;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheValue {
    Memcached { value: protocol_memcache::Value },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    value: CacheValue,
    expire_at: Instant,
}

impl CacheEntry {
    pub fn _expiry_epoch_seconds(&self) -> i64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                n.as_secs() as i64 + self.expire_at.duration_since(Instant::now()).as_secs() as i64
            }
            Err(_) => 0,
        }
    }

    pub fn into_value(self) -> CacheValue {
        self.value
    }
}

#[derive(Clone)]
pub struct MCache {
    cache: Arc<SegmentedCache<KeyType, CacheEntry, RandomState, BytesWeight>>,
    ttl: Duration,
}

#[derive(Debug, Clone, Copy)]
struct BytesWeight;

impl k_cache::Weigher<KeyType, CacheEntry> for BytesWeight {
    fn weigh(key: &KeyType, value: &CacheEntry) -> usize {
        key.len()
            + match &value.value {
                CacheValue::Memcached { value } => value.len().unwrap_or_default(),
            }
            + size_of::<protocol_memcache::Value>()
    }
}

type KeyType = Vec<u8>;

impl MCache {
    pub fn new(max_bytes: usize, ttl: Duration) -> Self {
        let cache =
            SegmentedCache::<KeyType, CacheEntry, RandomState, BytesWeight>::new(8, max_bytes);
        Self {
            cache: Arc::new(cache),
            ttl: std::cmp::min(ttl, Duration::from_secs(5 * 365 * 24 * 3600)),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<CacheEntry>
    where
        KeyType: Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.cache.get(&key)
    }

    pub fn set(&self, key: KeyType, value: impl Into<CacheValue>) {
        self.cache.put(
            key,
            CacheEntry {
                value: value.into(),
                expire_at: Instant::now() + self.ttl,
            },
        )
    }

    pub fn delete<Q>(&self, key: &Q) -> Option<CacheValue>
    where
        KeyType: Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        // todo
        // self.cache.remove(key).map(|e| e.value)
        None
    }
}

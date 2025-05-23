use std::{
    borrow::Borrow, mem::size_of, time::{Duration, Instant, SystemTime, UNIX_EPOCH}
};

use moka::{Expiry, sync::Cache};

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
    cache: Cache<KeyType, CacheEntry>,
    ttl: Duration,
}

fn weigh(key: &KeyType, value: &CacheEntry) -> u32 {
    (key.len()
        + match &value.value {
            CacheValue::Memcached { value } => value.len().unwrap_or_default(),
        }
        + size_of::<protocol_memcache::Value>()) as u32
}

struct MCacheExpiry;

type KeyType = Vec<u8>;

impl Expiry<KeyType, CacheEntry> for MCacheExpiry {
    /// Returns the duration of the expiration of the value that was just
    /// created.
    fn expire_after_create(
        &self,
        _key: &KeyType,
        value: &CacheEntry,
        current_time: Instant,
    ) -> Option<Duration> {
        Some(value.expire_at.saturating_duration_since(current_time))
    }

    fn expire_after_update(
        &self,
        _key: &KeyType,
        value: &CacheEntry,
        updated_at: Instant,
        _duration_until_expiry: Option<Duration>,
    ) -> Option<Duration> {
        Some(value.expire_at.saturating_duration_since(updated_at))
    }
}

impl MCache {
    pub fn new(max_bytes: usize, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_bytes as u64)
            .weigher(weigh)
            .expire_after(MCacheExpiry)
            .build();
        Self { cache, ttl: std::cmp::min(ttl, Duration::from_secs(5 * 365 * 24 * 3600)) }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<CacheEntry>
    where 
        KeyType: Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.cache.get(&key)
    }

    pub fn set(&self, key: KeyType, value: impl Into<CacheValue>) {
        self.cache.insert(
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
        self.cache.remove(key).map(|e| e.value)
    }
}

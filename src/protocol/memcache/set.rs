// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::cache::CacheValue;
use crate::klog::{klog_set, Status};
use crate::{Error, *};
use momento::cache::SetRequest;
use protocol_memcache::*;

pub async fn set(
    client: &mut CacheClient,
    cache_name: &str,
    request: &Set,
    flags: bool,
    memory_cache: Option<MCache>,
) -> Result<Response, Error> {
    SET.increment();

    if request.value().is_empty() {
        SET_EX.increment();
        return Ok(Response::client_error("empty values not supported"));
    }

    let key = (*request.key()).to_owned();

    // Recording length of passed in value for command logging purposes.
    // The value does not yet have flags embedded.
    let value_len = request.value().len();

    let value = if flags {
        let mut value = request.flags().to_be_bytes().to_vec();
        value.extend_from_slice(request.value());
        value
    } else {
        (*request.value()).to_owned()
    };

    if let Some(memory_cache) = &memory_cache {
        // On write, populate the local in-memory cache immediately.
        //
        // This complements the read-through logic:
        // - Writes update both memory cache and momento
        // - Reads check memory cache first, then fall back to momento on miss, and backfill the memory cache
        //
        // This ensures that:
        // (1) A proxy process restart doesn't degrade performance (cache warms on read)
        // (2) Multiple proxies each keep a warm local cache, even if writes are done by others
        let flags = if flags { request.flags() } else { 0 };
        let value = protocol_memcache::Value::new(&key, flags, None, &request.value());
        memory_cache.set(key.to_vec(), CacheValue::Memcached { value });
    }

    BACKEND_REQUEST.increment();

    let ttl = request
        .ttl()
        .get()
        .map(|ttl| Duration::from_secs(ttl.max(1) as u64));

    match timeout(
        Duration::from_millis(200),
        client.send_request(SetRequest::new(cache_name, key.clone(), value.clone()).ttl(ttl)),
    )
    .await
    {
        Ok(Ok(_result)) => {
            SET_STORED.increment();

            if request.noreply() {
                klog_set(
                    &key,
                    request.flags(),
                    request.ttl().get().unwrap_or(0),
                    value_len,
                    Status::Stored,
                    0,
                );

                Ok(Response::stored(true))
            } else {
                klog_set(
                    &key,
                    request.flags(),
                    ttl.map(|v| v.as_secs()).unwrap_or(0) as _,
                    value_len,
                    Status::Stored,
                    value_len,
                );

                Ok(Response::stored(false))
            }
        }
        Ok(Err(e)) => {
            BACKEND_EX.increment();

            SET_EX.increment();
            SESSION_SEND.increment();

            klog_set(
                &key,
                request.flags(),
                request.ttl().get().unwrap_or(0),
                value_len,
                Status::ServerError,
                0,
            );

            error!("backend error for set: {}", e);
            Ok(Response::server_error(format!("{e}")))
        }
        Err(_) => {
            // timeout
            BACKEND_EX.increment();
            BACKEND_EX_TIMEOUT.increment();

            SET_EX.increment();
            SESSION_SEND.increment();

            klog_set(
                &key,
                request.flags(),
                request.ttl().get().unwrap_or(0),
                value_len,
                Status::Timeout,
                0,
            );

            Ok(Response::server_error("backend timeout"))
        }
    }
}

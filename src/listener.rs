// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::*;
use momento::CacheClientBuilder;
use momento_proxy::Protocol;
use pelikan_net::{TCP_ACCEPT, TCP_CLOSE, TCP_CONN_CURR};

pub(crate) async fn listener(
    listener: TcpListener,
    client_builder: CacheClientBuilder<ReadyToBuild>,
    cache_name: String,
    protocol: Protocol,
    flags: bool,
    proxy_metrics: impl ProxyMetrics,
    memory_cache: Option<MCache>,
) {
    // Establishing a gRPC connection is expensive, so the client needs to be created outside the
    // loop and reused to avoid paying that cost with each request. A Momento client can handle 100
    // simultaneous requests per gRPC connection. Increase connection_count in the config to add
    // more connections.
    let client = client_builder.clone().build().unwrap_or_else(|e| {
        // Note: this will not happen since we validated the client build in the main thread already
        eprintln!("could not create cache client: {}", e);
        std::process::exit(1);
    });
    // this acts as our listener thread and spawns tasks for each client
    loop {
        // accept a new client
        if let Ok((socket, _)) = listener.accept().await {
            TCP_ACCEPT.increment();

            let client = client.clone();
            let cache_name = cache_name.clone();

            // spawn a task for managing requests for the client
            let proxy_metrics = proxy_metrics.clone();
            let memory_cache = memory_cache.clone();

            tokio::spawn(async move {
                TCP_CONN_CURR.increment();
                let _connection_metric = proxy_metrics.begin_connection();

                match protocol {
                    Protocol::Memcache => {
                        crate::frontend::handle_memcache_client(
                            socket,
                            client,
                            cache_name,
                            flags,
                            proxy_metrics,
                            memory_cache,
                        )
                        .await;
                    }
                    Protocol::Resp => {
                        crate::frontend::handle_resp_client(
                            socket,
                            client,
                            cache_name,
                            proxy_metrics,
                        )
                        .await;
                    }
                }

                TCP_CONN_CURR.decrement();
                TCP_CLOSE.increment();
            });
        }
    }
}

use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use goodmetrics::{GaugeFactory, HistogramHandle};

use super::util::{
    proxy_request_latency_error_histogram, proxy_request_latency_hit_histogram,
    proxy_request_latency_miss_histogram, proxy_request_latency_ok_histogram,
    proxy_request_latency_timeout_histogram,
};

/// Latency histogram handles for a single RPC type.
#[derive(Clone, Debug)]
pub struct RpcMetrics {
    rpc: &'static str,
    latency_ok: HistogramHandle,
    latency_error: HistogramHandle,
    latency_timeout: HistogramHandle,
    latency_miss: HistogramHandle,
    latency_hit_mcache: HistogramHandle,
    latency_hit_momento: HistogramHandle,
}

impl RpcMetrics {
    /// Creates histogram handles for the given RPC name.
    pub fn new(gauge_factory: &GaugeFactory, rpc: &'static str) -> Self {
        Self {
            rpc,
            latency_ok: proxy_request_latency_ok_histogram(gauge_factory, rpc),
            latency_error: proxy_request_latency_error_histogram(gauge_factory, rpc),
            latency_timeout: proxy_request_latency_timeout_histogram(gauge_factory, rpc),
            latency_miss: proxy_request_latency_miss_histogram(gauge_factory, rpc),
            latency_hit_mcache: proxy_request_latency_hit_histogram(gauge_factory, rpc, "mcache"),
            latency_hit_momento: proxy_request_latency_hit_histogram(gauge_factory, rpc, "momento"),
        }
    }

    /// Starts a new RPC call tracking guard for this metric.
    pub fn record_api_call(&self) -> RpcCallGuard {
        RpcCallGuard::new(
            self.rpc,
            self.latency_ok.clone(),
            self.latency_error.clone(),
            self.latency_timeout.clone(),
            self.latency_miss.clone(),
            self.latency_hit_mcache.clone(),
            self.latency_hit_momento.clone(),
        )
    }
}

/// Guard that records latency when a single RPC call completes or is dropped (timeout).
#[derive(Clone)]
pub struct RpcCallGuard {
    rpc: &'static str,
    start_time: Instant,
    latency_ok: HistogramHandle,
    latency_error: HistogramHandle,
    latency_timeout: HistogramHandle,
    latency_miss: HistogramHandle,
    latency_hit_mcache: HistogramHandle,
    latency_hit_momento: HistogramHandle,
    recorded: Arc<AtomicBool>,
}

impl RpcCallGuard {
    /// Creates a new guard, capturing the start time.
    pub fn new(
        rpc: &'static str,
        latency_ok: HistogramHandle,
        latency_error: HistogramHandle,
        latency_timeout: HistogramHandle,
        latency_miss: HistogramHandle,
        latency_hit_mcache: HistogramHandle,
        latency_hit_momento: HistogramHandle,
    ) -> Self {
        Self {
            rpc,
            start_time: Instant::now(),
            latency_ok,
            latency_error,
            latency_timeout,
            latency_miss,
            latency_hit_mcache,
            latency_hit_momento,
            recorded: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Records a successful RPC completion.
    pub fn complete_ok(&mut self) {
        if let Ok(false) =
            self.recorded
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            self.latency_ok
                .observe(self.start_time.elapsed().as_nanos() as i64);
        }
    }

    /// Records a failed RPC completion.
    pub fn complete_error(&mut self) {
        if let Ok(false) =
            self.recorded
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            self.latency_error
                .observe(self.start_time.elapsed().as_nanos() as i64);
        }
    }

    /// Records completion based on whether `result` is `Ok` or `Err`.
    pub fn complete<T, E>(&mut self, result: &Result<T, E>) {
        match result {
            Ok(_) => self.complete_ok(),
            Err(_) => self.complete_error(),
        };
    }

    /// Records a cache miss. May be called multiple times for multi-key requests.
    pub fn complete_miss(&mut self) {
        // Might observe multiple hits if multiple keys are requested, so update
        // the recorded flag but don't check it before recording the latency.
        debug!("{} complete_miss", self.rpc);
        let _ = self
            .recorded
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        self.latency_miss
            .observe(self.start_time.elapsed().as_nanos() as i64);
    }

    /// Records a hit served from the local in-process memory cache.
    pub fn complete_hit_mcache(&mut self) {
        // Might observe multiple hits if multiple keys are requested, so update
        // the recorded flag but don't check it before recording the latency.
        debug!("{} complete_hit_mcache", self.rpc);
        let _ = self
            .recorded
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        self.latency_hit_mcache
            .observe(self.start_time.elapsed().as_nanos() as i64);
    }

    /// Records a hit served from the Momento backend.
    pub fn complete_hit_momento(&mut self) {
        // Might observe multiple hits if multiple keys are requested, so update
        // the recorded flag but don't check it before recording the latency.
        debug!("{} complete_hit_momento", self.rpc);
        let _ = self
            .recorded
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        self.latency_hit_momento
            .observe(self.start_time.elapsed().as_nanos() as i64);
    }
}

impl Drop for RpcCallGuard {
    fn drop(&mut self) {
        if let Ok(false) =
            self.recorded
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            debug!("{} complete_timeout", self.rpc);
            self.latency_timeout
                .observe(self.start_time.elapsed().as_nanos() as i64);
        }
    }
}

/// Awaits `fut` and records its success or failure via `recorder`.
pub async fn with_rpc_call_guard<T, E, F>(mut recorder: RpcCallGuard, fut: F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    let result = fut.await;
    recorder.complete(&result);
    result
}

/// Implemented by response types that embed a protocol-level error.
pub trait ResponseWrappingError {
    /// Returns `true` if the response represents an error.
    fn is_error(&self) -> bool;
}

/// Awaits `fut` and records ok/error based on whether the successful response wraps an error.
pub async fn with_wrapped_error_response_rpc_call_guard<R: ResponseWrappingError, E, F>(
    mut recorder: RpcCallGuard,
    fut: F,
) -> Result<R, E>
where
    F: Future<Output = Result<R, E>>,
{
    let result = fut.await;
    match &result {
        Ok(response) => {
            if response.is_error() {
                recorder.complete_error();
            } else {
                recorder.complete_ok();
            }
        }
        Err(_) => {
            recorder.complete_error();
        }
    }
    result
}

use metriken::*;

/// Percentile labels and their corresponding quantile values used for histogram reporting.
pub static PERCENTILES: &[(&str, f64)] = &[
    ("p25", 25.0),
    ("p50", 50.0),
    ("p75", 75.0),
    ("p90", 90.0),
    ("p99", 99.0),
    ("p999", 99.9),
    ("p9999", 99.99),
];

/// Number of admin requests parsed.
#[metric(name = "admin_request_parse")]
pub static ADMIN_REQUEST_PARSE: Counter = Counter::new();

/// Number of admin responses composed.
#[metric(name = "admin_response_compose")]
pub static ADMIN_RESPONSE_COMPOSE: Counter = Counter::new();

/// Total backend (Momento) requests sent.
#[metric(name = "backend_request")]
pub static BACKEND_REQUEST: Counter = Counter::new();

/// Total backend errors.
#[metric(name = "backend_ex")]
pub static BACKEND_EX: Counter = Counter::new();

/// Backend errors due to rate limiting.
#[metric(name = "backend_ex_rate_limited")]
pub static BACKEND_EX_RATE_LIMITED: Counter = Counter::new();

/// Backend errors due to request timeout.
#[metric(name = "backend_ex_timeout")]
pub static BACKEND_EX_TIMEOUT: Counter = Counter::new();

/// User CPU time consumed (microseconds).
#[metric(name = "ru_utime")]
pub static RU_UTIME: Counter = Counter::new();

/// System CPU time consumed (microseconds).
#[metric(name = "ru_stime")]
pub static RU_STIME: Counter = Counter::new();

/// Maximum resident set size (kilobytes).
#[metric(name = "ru_maxrss")]
pub static RU_MAXRSS: Gauge = Gauge::new();

/// Integral shared memory size.
#[metric(name = "ru_ixrss")]
pub static RU_IXRSS: Gauge = Gauge::new();

/// Integral unshared data size.
#[metric(name = "ru_idrss")]
pub static RU_IDRSS: Gauge = Gauge::new();

/// Integral unshared stack size.
#[metric(name = "ru_isrss")]
pub static RU_ISRSS: Gauge = Gauge::new();

/// Number of minor (reclaimed) page faults.
#[metric(name = "ru_minflt")]
pub static RU_MINFLT: Counter = Counter::new();

/// Number of major (I/O-requiring) page faults.
#[metric(name = "ru_majflt")]
pub static RU_MAJFLT: Counter = Counter::new();

/// Number of times the process was swapped out.
#[metric(name = "ru_nswap")]
pub static RU_NSWAP: Counter = Counter::new();

/// Number of block input operations.
#[metric(name = "ru_inblock")]
pub static RU_INBLOCK: Counter = Counter::new();

/// Number of block output operations.
#[metric(name = "ru_oublock")]
pub static RU_OUBLOCK: Counter = Counter::new();

/// Number of IPC messages sent.
#[metric(name = "ru_msgsnd")]
pub static RU_MSGSND: Counter = Counter::new();

/// Number of IPC messages received.
#[metric(name = "ru_msgrcv")]
pub static RU_MSGRCV: Counter = Counter::new();

/// Number of signals received.
#[metric(name = "ru_nsignals")]
pub static RU_NSIGNALS: Counter = Counter::new();

/// Number of voluntary context switches.
#[metric(name = "ru_nvcsw")]
pub static RU_NVCSW: Counter = Counter::new();

/// Number of involuntary context switches.
#[metric(name = "ru_nivcsw")]
pub static RU_NIVCSW: Counter = Counter::new();

mod builder;
mod connection;
mod proxy;
mod rpc;
/// Gauge factory helper utilities.
pub mod util;

pub use builder::ProxyMetricsBuilder;
pub use connection::ConnectionGuard;
pub use proxy::{
    ConnectionMetrics, DefaultProxyMetrics, MemcachedMetrics, ProxyMetrics, RespMetrics,
};
pub use rpc::{
    with_rpc_call_guard, with_wrapped_error_response_rpc_call_guard, ResponseWrappingError,
    RpcCallGuard, RpcMetrics,
};

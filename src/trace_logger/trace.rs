use std::fmt::Display;

use clocksource::datetime::DateTime;
use config::DebugConfig;
pub use ringlog::*;

use crate::{klog::EscapedStr, trace_logger::config::TraceConfig};

pub static START_TIME: std::sync::OnceLock<std::time::SystemTime> = std::sync::OnceLock::new();

fn trace_format(
    w: &mut dyn std::io::Write,
    _now: DateTime,
    record: &Record,
) -> Result<(), std::io::Error> {
    writeln!(w, "{}", record.args())
}

pub fn configure_logging<T: DebugConfig + TraceConfig>(config: &T) -> Box<dyn Drain> {
    let debug_config = config.debug();

    let debug_output: Box<dyn Output> = if let Some(file) = debug_config.log_file() {
        let backup = debug_config.log_backup().unwrap_or(format!("{file}.old"));
        Box::new(
            File::new(&file, &backup, debug_config.log_max_size())
                .expect("failed to open debug log file"),
        )
    } else {
        Box::new(Stdout::new())
    };

    let debug_log = LogBuilder::new()
        .output(debug_output)
        .log_queue_depth(debug_config.log_queue_depth())
        .single_message_size(debug_config.log_single_message_size())
        .build()
        .expect("failed to initialize debug log");
    let trace_config = config.trace();
    let trace_output: Box<dyn Output> = if let Some(file) = trace_config.file() {
        let backup = trace_config.backup().unwrap_or(format!("{file}.old"));
        Box::new(
            File::new(&file, &backup, trace_config.max_size())
                .expect("failed to open cache-trace log file"),
        )
    } else {
        Box::new(Stdout::new())
    };

    let trace_log = LogBuilder::new()
        .output(trace_output)
        .format(trace_format)
        .log_queue_depth(trace_config.queue_depth())
        .single_message_size(trace_config.single_message_size())
        .build()
        .expect("failed to initialize cache-trace log");

    START_TIME
        .set(std::time::SystemTime::now())
        .expect("failed to set start time for cache-trace logger");

    MultiLogBuilder::new()
        .level_filter(debug_config.log_level().to_level_filter())
        .default(debug_log)
        .add_target("trace", trace_log)
        .add_target("klog", NopLogBuilder::new().build())
        .build()
        .start()
}

#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => (
        // we choose error level here because it is the lowest level and will
        // not be filtered unless the level filter is set to `off`
        error!(target: "trace", $($arg)*);
    )
}

// Expected format for memcached proxy cache-trace output: timestamp, key, key size, value size, client id, operation, ttl
// We will just use client id of 0 for now
pub(crate) fn trace_command(
    command: &dyn Display,
    key: &dyn AsRef<[u8]>,
    value_size: usize,
    ttl: i32,
) {
  if let Some(start_time) = START_TIME.get() {
    let timestamp_seconds = std::time::SystemTime::now()
        .duration_since(*start_time)
        .unwrap()
        .as_secs();
    let key_size = key.as_ref().len();
    trace_log!(
        "{},{},{},{},{},{},{}",
        timestamp_seconds,
        EscapedStr::new(key),
        key_size,
        value_size,
        0,
        command,
        ttl
    );
  }
}

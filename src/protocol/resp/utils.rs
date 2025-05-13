// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use momento::MomentoError;
use std::future::Future;

use crate::error::ProxyError;

pub(crate) fn momento_error_to_resp_error(buf: &mut Vec<u8>, command: &str, error: MomentoError) {
    use crate::BACKEND_EX;

    BACKEND_EX.increment();

    error!("backend error for {command}: {error}");
    buf.extend_from_slice(format!("-ERR backend error: {error}\r\n").as_bytes());
}

pub(crate) async fn update_method_metrics<T, E>(
    count: &metriken::Counter,
    count_ex: &metriken::Counter,
    future: impl Future<Output = Result<T, E>>,
) -> Result<T, E> {
    count.increment();
    future.await.inspect_err(|_| {
        count_ex.increment();
    })
}

pub(crate) fn parse_score_boundary_as_integer(value: &[u8]) -> Result<i32, ProxyError> {
    let index = std::str::from_utf8(value)
        .map_err(|_| {
            ProxyError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ZRANGE index is not valid utf8",
            ))
        })?
        .parse::<i32>()
        .map_err(|_| {
            ProxyError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ZRANGE index is not an integer",
            ))
        })?;
    Ok(index)
}

// Returns a tuple of (value, is_exclusive)
pub(crate) fn parse_score_boundary_as_float(value: &[u8]) -> Result<(f64, bool), ProxyError> {
    // First check if the value is +inf or -inf
    if value == b"+inf" {
        return Ok((f64::INFINITY, false));
    }
    if value == b"-inf" {
        return Ok((f64::NEG_INFINITY, false));
    }

    // Otherwise, split apart exclusive symbol '(' and the value if present
    let (exclusive_symbol, number) = if value[0] == b'(' {
        (true, &value[1..])
    } else {
        (false, value)
    };

    let score = std::str::from_utf8(number)
        .map_err(|_| {
            ProxyError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ZRANGE score is not valid utf8",
            ))
        })?
        .parse::<f64>()
        .map_err(|_| {
            ProxyError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ZRANGE score is not a float",
            ))
        })?;

    if exclusive_symbol {
        Ok((score, true))
    } else {
        Ok((score, false))
    }
}

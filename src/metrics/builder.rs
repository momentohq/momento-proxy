use goodmetrics::{
    default_gauge_factory,
    downstream::{get_client, OpenTelemetryDownstream, OpentelemetryBatcher},
    pipeline::DimensionPosition,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_rustls::rustls::RootCertStore;
use tonic::metadata::MetadataValue;

use super::proxy::DefaultProxyMetrics;

/// Builder for constructing an [`Arc<DefaultProxyMetrics>`] with an optional OTLP downstream.
pub struct ProxyMetricsBuilder {
    batch_interval: Duration,
    batch_capacity: usize,
}

impl Default for ProxyMetricsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProxyMetricsBuilder {
    /// Creates a builder with default settings (1-second batch interval, 128-item capacity).
    pub fn new() -> Self {
        Self {
            batch_interval: Duration::from_secs(1),
            batch_capacity: 128,
        }
    }

    /// Builds the metrics, optionally wiring up an OTLP downstream from environment variables.
    pub async fn build(self) -> Arc<DefaultProxyMetrics> {
        let (batch_sender, batch_receiver) = mpsc::channel(self.batch_capacity);
        let gauge_factory = default_gauge_factory();

        let endpoint = get_environment_variable_or_none("OTLP_ENDPOINT");
        let api_token = get_environment_variable_or_none("OTLP_API_TOKEN");

        match (&endpoint, &api_token) {
            (Some(endpoint), Some(api_token)) => {
                info!("Configuring OTLP downstream with provided endpoint and API token");

                let channel = get_client(
                    endpoint,
                    || {
                        Some(RootCertStore {
                            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec()
                        })
                    },
                    goodmetrics::proto::opentelemetry::collector::metrics::v1::metrics_service_client::MetricsServiceClient::with_origin
                );
                let api_token_metadata = MetadataValue::try_from(api_token);

                match (channel, api_token_metadata) {
                    (Ok(channel), Ok(api_token_metadata)) => {
                        let otlp_downstream = OpenTelemetryDownstream::new_with_dimensions(
                            channel,
                            Some(("api-token", api_token_metadata)),
                            get_base_environment_dimensions(),
                        );
                        tokio::spawn(otlp_downstream.send_batches_forever(batch_receiver));
                        tokio::spawn(gauge_factory.clone().report_gauges_forever(
                            self.batch_interval,
                            batch_sender,
                            OpentelemetryBatcher,
                        ));
                    }
                    (Err(e), _) => {
                        warn!(
                            "Failed to create OTLP client, not configuring OTLP downstream: {}",
                            e
                        );
                    }
                    (_, Err(e)) => {
                        warn!(
                            "Failed to parse OTLP API token, not configuring OTLP downstream: {}",
                            e
                        );
                    }
                }
            }
            (None, _) => {
                info!("OTLP endpoint not provided: not configuring OTLP downstream. Set the OTLP_ENDPOINT environment variable to configure.");
            }
            (_, None) => {
                info!("OTLP API token not provided, not configuring OTLP downstream. Set the OTLP_API_TOKEN environment variable to configure.");
            }
        }

        let metrics = DefaultProxyMetrics::new(gauge_factory, self.batch_interval);
        Arc::new(metrics)
    }
}

fn get_base_environment_dimensions() -> DimensionPosition {
    DimensionPosition::from_iter(
        vec![
            // We require a standard Otel Collector `service.instance.id` and `service.name`
            // dimensions in order to ingest metrics, otherwise they are rejected.
            // We don't really "need" a distinct value, we just need something, which
            // will default to `unknown`. If we need something in the future, we can
            // add that as necessary.
            (
                "service.instance.id",
                get_environment_variable("SERVICE_INSTANCE_ID"),
            ),
            ("service.name", get_environment_variable("SERVICE_NAME")),
        ]
        .into_iter()
        .map(|(n, v)| (n.into(), v.into())),
    )
}

fn get_environment_variable(variable: &str) -> String {
    match std::env::var(variable) {
        Ok(val) => val,
        Err(_) => {
            info!(
                "Environment variable {} not set, defaulting to 'unknown'",
                variable
            );
            "unknown".to_string()
        }
    }
}

fn get_environment_variable_or_none(variable: &str) -> Option<String> {
    std::env::var(variable).ok()
}

mod stream;
mod summary;

use crate::{AppError, SrsConfig};
use prometheus::{Encoder, Registry, TextEncoder};
use std::result::Result;
use stream::StreamCollector;
use summary::SummaryCollector;

/**
 * The Metric Collector, wraps a set of collectors
 */
#[derive(Clone, Debug)]
pub struct MetricCollector {
    registry: Registry,
    stream: StreamCollector,
    summary: SummaryCollector,
}

impl MetricCollector {
    /**
     * Constructor
     */
    pub fn new(registry: Registry, srs_config: SrsConfig) -> Self {
        let stream = StreamCollector::new(&registry, &srs_config);
        let summary = SummaryCollector::new(&registry, &srs_config);
        Self {
            registry,
            stream,
            summary,
        }
    }

    /**
     * Gather the metrics.
     */
    pub async fn collect(self) -> Result<String, AppError> {
        self.stream.collect().await?;
        self.summary.collect().await?;
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Ok(String::from_utf8(buffer).unwrap())
    }
}

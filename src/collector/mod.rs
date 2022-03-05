mod stream;
mod summary;

// TODO more collector
use crate::SrsConfig;
use prometheus::{Encoder, Registry, TextEncoder};
use stream::StreamCollector;
use summary::SummaryCollector;

#[derive(Clone, Debug)]
pub struct MetricCollector {
    registry: Registry,
    stream: StreamCollector,
    summary: SummaryCollector,
}

impl MetricCollector {
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
    pub async fn collect(self) -> String {
        self.stream.collect().await;
        self.summary.collect().await;
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

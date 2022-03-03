use anyhow::Result;
use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};

use super::Collector;

const BASE_URL: &str = "/api/v1/streams/";

pub struct StreamUsage {
    registry: Registry,
    total: Gauge,
    clients: Gauge,
}

impl StreamUsage {
    pub fn new(registry: Registry) -> Self {
        let su = Self {
            registry: registry,
            total: Gauge::with_opts(Opts::new(
                "stream_active_total",
                "Total amount of active streams",
            ))
            .unwrap(),
            clients: Gauge::with_opts(Opts::new("stream_clients", "connected clients")).unwrap(),
        };
        // 在初始化时注册到 registry
        su.registry.register(Box::new(su.total.clone())).unwrap();
        su.registry.register(Box::new(su.clients.clone())).unwrap();
        su
    }
}

impl Collector for StreamUsage {
    fn collect(&self) -> Result<String> {
        self.total.add(12.0);
        self.clients.add(3.0);
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Ok(String::from_utf8(buffer).unwrap())
    }
}

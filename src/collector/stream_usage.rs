use crate::SrsConfig;
use anyhow::Result;
use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};
use serde_derive::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str = "/api/v1/streams/";

pub struct StreamUsage {
    registry: Registry,
    srs_url: String,
    total: Gauge,
    clients: Gauge,
}

#[derive(Deserialize)]
struct StreamResponse {
    code: i16,
    server: String,
    // streams: Vec<StreamStatus>,
}

struct StreamStatus {
    id: String,
    name: String,
    vhost: String,
    app: String,
    kps: (),
    publish: (),
}

impl StreamUsage {
    pub fn new(registry: Registry, srs_config: &SrsConfig) -> Self {
        let srs_url = format!(
            "http://{}:{}{}",
            srs_config.host, srs_config.http_port, BASE_URL
        );
        let su = Self {
            registry: registry,
            srs_url,
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

    pub async fn collect(&self) -> Result<String> {
        // get current stream usage
        let ret = reqwest::get(self.srs_url.clone())
            .await?
            .json::<HashMap<String, String>>()
            .await?;
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

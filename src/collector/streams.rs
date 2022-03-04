use crate::SrsConfig;
use anyhow::Result;
use prometheus::{Encoder, IntGauge, Opts, Registry, TextEncoder};
use serde_derive::Deserialize;

const BASE_URL: &str = "/api/v1/streams/";

/**
 * manage all streams or specified stream
 */
pub struct StreamCollector {
    registry: Registry,
    srs_url: String,
    total: IntGauge,
    clients: IntGauge,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
struct StreamResponse {
    code: i16,
    server: String,
    streams: Vec<StreamStatus>,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
struct StreamStatus {
    // 对齐
    clients: u32,
    frames: u32,
    send_bytes: u32,
    recv_bytes: u32,
    live_ms: u64,
    id: String,
    name: String,
    vhost: String,
    app: String,
}

impl StreamCollector {
    pub fn new(registry: Registry, srs_config: &SrsConfig) -> Self {
        let srs_url = format!(
            "http://{}:{}{}",
            srs_config.host, srs_config.http_port, BASE_URL
        );
        let su = Self {
            registry: registry,
            srs_url,
            total: IntGauge::with_opts(Opts::new(
                "stream_active_total",
                "Total amount of active streams",
            ))
            .unwrap(),
            clients: IntGauge::with_opts(Opts::new(
                "stream_clients_total",
                "Total amount of connected clients",
            ))
            .unwrap(),
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
            .json::<StreamResponse>()
            .await?;
        println!("Stream Response: {:?}", ret);
        self.total.set(ret.streams.len() as i64);
        let mut total_clients: i64 = 0;
        for s in ret.streams.into_iter() {
            total_clients += s.clients as i64;
        }
        self.clients.set(total_clients);
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Ok(String::from_utf8(buffer).unwrap())
    }
}

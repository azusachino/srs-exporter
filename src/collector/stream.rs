use prometheus::{IntGauge, Opts, Registry};
use serde_derive::Deserialize;

use crate::SrsConfig;

const BASE_URL: &str = "/api/v1/streams/";

/**
 * manage all streams or specified stream
 */
#[derive(Clone, Debug)]
pub struct StreamCollector {
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
    pub fn new(registry: &Registry, srs_config: &SrsConfig) -> Self {
        let su = Self {
            srs_url: format!(
                "http://{}:{}{}",
                srs_config.host,
                srs_config.http_port.unwrap(),
                BASE_URL
            ),
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
        registry.register(Box::new(su.total.clone())).unwrap();
        registry.register(Box::new(su.clients.clone())).unwrap();
        su
    }

    pub async fn collect(self) {
        // get current stream usage
        let ret = reqwest::Client::new()
            .get(self.srs_url)
            .send()
            .await
            .unwrap()
            .json::<StreamResponse>()
            // .text()
            .await
            .unwrap();
        // println!("Stream Response: {:?}", ret);
        self.total.set(ret.streams.len() as i64);
        let mut total_clients: i64 = 0;
        if ret.streams.len() > 0 {
            for s in ret.streams.into_iter() {
                total_clients += s.clients as i64;
            }
        }
        self.clients.set(total_clients);
    }
}

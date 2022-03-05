use prometheus::Registry;
use srs_exporter::{MetricCollector, SrsConfig};

// cargo test --package srs_exporter --test stream_collector_test
#[tokio::main]
async fn main() {
    let r = Registry::new();
    let srs_config = SrsConfig {
        host: "localhost".to_string(),
        http_port: Some(1985),
        rtmp_port: Some(1935),
    };
    let mc = MetricCollector::new(r, srs_config);

    let body = mc.collect().await;
    println!("{}", body)
}

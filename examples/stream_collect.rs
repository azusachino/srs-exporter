use prometheus::Registry;
use srs_exporter::{AppError, MetricCollector, SrsConfig};

// cargo test --package srs_exporter --test stream_collector_test
#[tokio::main]
async fn main() -> std::result::Result<(), AppError> {
    let r = Registry::new();
    let srs_config = SrsConfig::default();
    let mc = MetricCollector::new(r, srs_config);

    let body = mc.collect().await?;
    println!("{}", body);
    Ok(())
}

/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
use anyhow::Result;
use serde_derive::Deserialize;

mod collector;
pub use collector::StreamUsage;

pub const CURRENT_VERSION: &str = "0.0.1";

const CONFIG_LOCATION: &str = "config.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct SrsExporterConfig {
    srs_host: String,
    srs_http_port: String,
    srs_rtmp_port: String,
    nacos_host: String,
    nacos_port: String,
}

pub fn parse_config() -> Result<SrsExporterConfig> {
    use std::env;
    let read_bytes = std::fs::read(CONFIG_LOCATION).unwrap();
    // try read from config
    let mut toml_config: SrsExporterConfig = toml::from_slice(&read_bytes).unwrap();
    // check exists, if not try read from env
    if toml_config.srs_host.is_empty() {
        toml_config.srs_host = env::var("SRS_HOST")?;
    }
    if toml_config.srs_http_port.is_empty() {
        toml_config.srs_http_port = env::var("SRS_HTTP_PORT")?;
    }
    if toml_config.srs_rtmp_port.is_empty() {
        toml_config.srs_rtmp_port = env::var("SRS_RTMP_PORT")?;
    }
    if toml_config.nacos_host.is_empty() {
        toml_config.nacos_host = env::var("NACOS_HOST")?;
    }
    if toml_config.nacos_port.is_empty() {
        toml_config.nacos_port = env::var("NACOS_PORT")?;
    }
    Ok(toml_config)
}

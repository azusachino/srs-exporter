/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
use anyhow::Result;
use serde_derive::Deserialize;
use std::env;

mod collector;
pub use collector::StreamCollector;

mod nacos;
pub use nacos::NacosClient;

pub const DEFAULT_CONFIG: &str = "config.toml";
pub const CURRENT_VERSION: &str = "0.0.1";

#[derive(Clone, Debug, Deserialize)]
pub struct SrsExporterConfig {
    pub srs: SrsConfig,
    pub nacos: NacosConfig,
}

/**
 * SRS Config
 */
#[derive(Clone, Debug, Deserialize)]
pub struct SrsConfig {
    host: String,
    http_port: String,
    rtmp_port: String,
}

/**
 * SRS Config
 */
#[derive(Clone, Debug, Deserialize)]
pub struct NacosConfig {
    host: String,
    port: String,
    namespace_id: String,
    group_name: String,
}

pub fn parse_config(config: String) -> Result<SrsExporterConfig> {
    let read_bytes = std::fs::read_to_string(config)?;
    // try read from config
    let mut toml_config: SrsExporterConfig = toml::from_str(&read_bytes)?;

    // check exists, if not try read from env
    if toml_config.srs.host.is_empty() {
        match env::var("SRS_HOST") {
            Ok(host) => toml_config.srs.host = host,
            Err(_) => {
                toml_config.srs.host = String::from("localhost");
            }
        }
    }
    if toml_config.srs.http_port.is_empty() {
        match env::var("SRS_HTTP_PORT") {
            Ok(http_port) => toml_config.srs.http_port = http_port,
            Err(_) => {
                toml_config.srs.http_port = String::from("1985");
            }
        }
    }
    if toml_config.srs.rtmp_port.is_empty() {
        match env::var("SRS_RTMP_PORT") {
            Ok(rtmp_port) => toml_config.srs.rtmp_port = rtmp_port,
            Err(_) => {
                toml_config.srs.rtmp_port = String::from("1935");
            }
        }
    }
    if toml_config.nacos.host.is_empty() {
        match env::var("NACOS_HOST") {
            Ok(host) => toml_config.nacos.host = host,
            Err(_) => {
                toml_config.nacos.host = String::from("localhost");
            }
        }
    }
    if toml_config.nacos.port.is_empty() {
        match env::var("NACOS_PORT") {
            Ok(port) => toml_config.nacos.port = port,
            Err(_) => {
                toml_config.nacos.port = String::from("8848");
            }
        }
    }
    if toml_config.nacos.namespace_id.is_empty() {
        match env::var("NACOS_NAMESPACE_ID") {
            Ok(namespace_id) => toml_config.nacos.namespace_id = namespace_id,
            Err(_) => {
                toml_config.nacos.namespace_id = String::from("public");
            }
        }
    }
    if toml_config.nacos.group_name.is_empty() {
        match env::var("NACOS_HOST") {
            Ok(group_name) => toml_config.nacos.group_name = group_name,
            Err(_) => {
                toml_config.nacos.group_name = String::from("DEFAULT_GROUP");
            }
        }
    }

    Ok(toml_config)
}

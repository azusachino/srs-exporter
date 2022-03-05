/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
use anyhow::Result;
use serde_derive::Deserialize;
use std::env;

pub use collector::MetricCollector;
pub use nacos::NacosClient;

mod collector;
mod nacos;

pub const DEFAULT_CONFIG: &str = "config.toml";
pub const CURRENT_VERSION: &str = "0.0.2";

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SrsExporterConfig {
    pub port: Option<u16>,
    pub srs: SrsConfig,
    pub nacos: NacosConfig,
}

/**
 * SRS Config
 */
#[derive(Clone, Debug, Deserialize, Default)]
pub struct SrsConfig {
    pub http_port: Option<u16>,
    pub rtmp_port: Option<u16>,
    pub host: String,
}

/**
 * Nacos Config
 */
#[derive(Clone, Debug, Deserialize, Default)]
pub struct NacosConfig {
    pub port: Option<u16>,
    pub host: String,
    pub namespace_id: String,
    pub group_name: String,
}

/**
 * Parse config from config.toml
 */
pub fn parse_config(config: String) -> Result<SrsExporterConfig> {
    // try read from config
    let mut toml_config: SrsExporterConfig = match std::fs::read_to_string(config) {
        Ok(string) => toml::from_str(&string)?,
        // no config file, create default
        Err(_) => SrsExporterConfig::default(),
    };

    // check config exists, if not try read from env
    match toml_config.port {
        Some(_) => {}
        None => match env::var("EXPORTER_PORT") {
            Ok(port) => toml_config.port = Some(port.parse::<u16>().unwrap()),
            Err(_) => {
                toml_config.port = Some(9717);
            }
        },
    }

    if toml_config.srs.host.is_empty() {
        match env::var("SRS_HOST") {
            Ok(host) => toml_config.srs.host = host,
            Err(_) => {
                toml_config.srs.host = String::from("localhost");
            }
        }
    }

    match toml_config.srs.http_port {
        Some(_) => {}
        None => match env::var("SRS_HTTP_PORT") {
            Ok(http_port) => toml_config.srs.http_port = Some(http_port.parse::<u16>().unwrap()),
            Err(_) => {
                toml_config.srs.http_port = Some(1985);
            }
        },
    }

    match toml_config.srs.rtmp_port {
        Some(_) => {}
        None => match env::var("SRS_RTMP_PORT") {
            Ok(rtmp_port) => toml_config.srs.rtmp_port = Some(rtmp_port.parse::<u16>().unwrap()),
            Err(_) => {
                toml_config.srs.rtmp_port = Some(1935);
            }
        },
    }

    if toml_config.nacos.host.is_empty() {
        match env::var("NACOS_HOST") {
            Ok(host) => toml_config.nacos.host = host,
            Err(_) => {
                toml_config.nacos.host = String::from("localhost");
            }
        }
    }

    match toml_config.nacos.port {
        Some(_) => {}
        None => match env::var("SRS_RTMP_PORT") {
            Ok(port) => toml_config.nacos.port = Some(port.parse::<u16>().unwrap()),
            Err(_) => {
                toml_config.nacos.port = Some(8848);
            }
        },
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

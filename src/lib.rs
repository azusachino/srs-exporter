/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
use anyhow::Result;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
pub use collector::MetricCollector;
pub use nacos::NacosClient;
use serde_derive::Deserialize;
use serde_json::json;
use std::env;

mod collector;
mod nacos;

pub const DEFAULT_CONFIG: &str = "config.toml";
pub const CURRENT_VERSION: &str = "0.0.3";

// Erros that can happen
#[derive(Debug)]
pub enum AppError {
    NacosUnreachable,
    SrsUnreachable,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NacosUnreachable => (StatusCode::INTERNAL_SERVER_ERROR,  "Cannot reach Nacos server, please check whether srs is healthy or whether the http api is configured"),
            AppError::SrsUnreachable =>  (StatusCode::INTERNAL_SERVER_ERROR, "Cannot reach Srs server, please check whether srs is healthy or whether the http api is configured")
        };

        let body = Json(json!({ "error": msg }));

        (status, body).into_response()
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SrsExporterConfig {
    pub app: AppConfig,
    pub srs: SrsConfig,
    pub nacos: NacosConfig,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    /**
     * Srs Exporter Running port [will report to nacos]
     */
    pub port: Option<u16>,
    /**
     * Srs Exporter's host [will report to nacos]
     */
    pub host: String,
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
    if toml_config.app.host.is_empty() {
        match env::var("SRS_EXPORTER_HOST") {
            Ok(host) => toml_config.app.host = host,
            Err(_) => {
                toml_config.app.host = String::from("localhost");
            }
        }
    }

    match toml_config.app.port {
        Some(_) => {}
        None => match env::var("SRS_EXPORTER_PORT") {
            Ok(port) => toml_config.app.port = Some(port.parse::<u16>().unwrap()),
            Err(_) => {
                toml_config.app.port = Some(9717);
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

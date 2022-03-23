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
use std::fmt::{self, Display};

mod collector;
mod nacos;

pub const DEFAULT_CONFIG: &str = "config.toml";
pub const CURRENT_VERSION: &str = "1.0.0";

const NACOS_ERROR_MSG: &str =
    "Cannot reach Nacos server, please check srs-exporter's config.toml and the Nacos server";
const SRS_ERROR_MSG: &str =
    "Cannot reach SRS server, please check SRS's configuration and the SRS Server";

// Errors that could happen
#[derive(Debug)]
pub enum AppError {
    NacosUnreachable,
    SrsUnreachable,
}

/**
 * HTTP Response
 */
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NacosUnreachable => (StatusCode::INTERNAL_SERVER_ERROR, NACOS_ERROR_MSG),
            AppError::SrsUnreachable => (StatusCode::INTERNAL_SERVER_ERROR, SRS_ERROR_MSG),
        };

        (status, Json(json!({ "tip": msg }))).into_response()
    }
}

/**
 * Println
 */
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::NacosUnreachable => write!(f, "{}", NACOS_ERROR_MSG),
            AppError::SrsUnreachable => write!(f, "{}", SRS_ERROR_MSG),
        }
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
    pub port: u16,
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
    /**
     * Srs host
     */
    pub host: String,
    /**
     * origin or edge [will report to nacos]
     */
    pub mode: String,
    pub rtmp_port: u16,
    pub http_port: u16,
}

/**
 * Nacos Config
 */
#[derive(Clone, Debug, Deserialize, Default)]
pub struct NacosConfig {
    pub port: u16,
    pub host: String,
    pub namespace_id: String,
    pub group_name: String,
}

/**
 * Parse config from config.toml
 * Update: remove all stupid param checks
 */
pub fn parse_config(config: String) -> Result<SrsExporterConfig> {
    // try read from config
    let toml_config: SrsExporterConfig = match std::fs::read_to_string(config) {
        Ok(string) => toml::from_str(&string)?,
        // no config file, create default
        Err(_) => SrsExporterConfig::default(),
    };

    Ok(toml_config)
}

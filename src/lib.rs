//! SRS Exporter
//!
//! Fetch SRS Status by http request, integrate with prometheus client.

use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserializer, Serializer};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use toml::value::Table;
use toml::{self, Value};

pub use collector::MetricCollector;
pub use nacos::NacosClient;

use crate::utils::TomlExt;

mod collector;
mod nacos;
mod utils;

pub const DEFAULT_CONFIG: &str = "config.toml";
/// The current version of `tsubame`
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

const NACOS_ERROR_MSG: &str =
    "Cannot reach Nacos server, Please check Nacos configuration and the Nacos server";
const SRS_ERROR_MSG: &str =
    "Cannot reach SRS server, Please check SRS configuration and the SRS Server";

// Errors that might happen
#[derive(Debug)]
pub enum AppError {
    // Nacos 无法连接
    NacosUnreachable,
    // SRS 无法连接
    SrsUnreachable,
}

/**
 * HTTP Response Wrapper for AppError
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
 * Println Wrapper for AppError
 */
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::NacosUnreachable => write!(f, "{}", NACOS_ERROR_MSG),
            AppError::SrsUnreachable => write!(f, "{}", SRS_ERROR_MSG),
        }
    }
}

/// SRS Exporter Configuration
#[derive(Clone, Debug, PartialEq)]
pub struct SrsExporterConfig {
    // current app configuration
    pub app: AppConfig,
    // srs configuration
    pub srs: SrsConfig,
    // nacos configuration
    pub nacos: NacosConfig,
    rest: Value,
}

impl Default for SrsExporterConfig {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
            srs: SrsConfig::default(),
            nacos: NacosConfig::default(),
            rest: Value::Table(Table::default()),
        }
    }
}

impl FromStr for SrsExporterConfig {
    type Err = anyhow::Error;

    fn from_str(src: &str) -> Result<Self> {
        toml::from_str(src).with_context(|| "Invalid configuration file")
    }
}

impl SrsExporterConfig {
    /**
     * Parse config from config.toml
     * Update: remove all stupid param checks
     */
    pub fn from_disk<P: AsRef<Path>>(config_file: P) -> Result<Self> {
        let mut buffer = String::new();
        File::open(config_file)
            .with_context(|| "Unable to open the configuration file")?
            .read_to_string(&mut buffer)
            .with_context(|| "Couldn't read the file")?;

        SrsExporterConfig::from_str(&buffer)
    }
}

impl<'de> serde::Deserialize<'de> for SrsExporterConfig {
    fn deserialize<D: Deserializer<'de>>(de: D) -> std::result::Result<Self, D::Error> {
        let raw = Value::deserialize(de)?;
        use serde::de::Error;

        let mut table = match raw {
            Value::Table(t) => t,
            _ => {
                return Err(Error::custom("A config file should always be a toml table"));
            }
        };

        let app: AppConfig = table
            .remove("app")
            .map(|app| app.try_into().map_err(Error::custom))
            .transpose()?
            .unwrap_or_default();
        let srs: SrsConfig = table
            .remove("srs")
            .map(|srs| srs.try_into().map_err(Error::custom))
            .transpose()?
            .unwrap_or_default();
        let nacos: NacosConfig = table
            .remove("nacos")
            .map(|nacos| nacos.try_into().map_err(Error::custom))
            .transpose()?
            .unwrap_or_default();

        Ok(SrsExporterConfig {
            app,
            srs,
            nacos,
            rest: Value::Table(table),
        })
    }
}

impl serde::Serialize for SrsExporterConfig {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        let mut table = self.rest.clone();

        let app_config = Value::try_from(&self.app).expect("should always be serializable");
        table.insert("app", app_config);

        let srs_config = Value::try_from(&self.srs).expect("should always be serializable");
        table.insert("srs", srs_config);

        let nacos_config = Value::try_from(&self.nacos).expect("should always be serializable");
        table.insert("nacos", nacos_config);

        table.serialize(s)
    }
}

/**
 * App Config
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 9707,
            host: String::from("127.0.0.1"),
        }
    }
}

/**
 * SRS Config
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SrsConfig {
    /**
     * origin or edge [will report to nacos]
     */
    pub mode: String,
    /**
     * Srs host for external service
     */
    pub domain: String,
    pub rtmp_port: u16,
    /**
     * Srs host for internal service
     */
    pub host: String,
    pub http_port: u16,
}

impl Default for SrsConfig {
    fn default() -> Self {
        Self {
            mode: String::from("edge"),
            domain: String::from("127.0.0.1"),
            rtmp_port: 1935,
            host: String::from("127.0.0.1"),
            http_port: 1985,
        }
    }
}

/**
 * Nacos Config
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NacosConfig {
    // 是否开启认证
    pub auth: bool,
    pub host: String,
    pub port: u16,
    pub namespace_id: String,
    pub group_name: String,
    pub username: String,
    pub password: String,
}

impl Default for NacosConfig {
    fn default() -> Self {
        Self {
            auth: false,
            host: String::from("127.0.0.1"),
            port: 8848,
            namespace_id: String::from("public"),
            group_name: String::from("public"),
            username: String::from("nacos"),
            password: String::from("nacos"),
        }
    }
}

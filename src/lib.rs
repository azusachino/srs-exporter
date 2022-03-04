/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
use anyhow::Result;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde_derive::Deserialize;

mod collector;
pub use collector::StreamUsage;

// const CONFIG_LOCATION: &str = "config.toml";
const CONFIG_LOCATION: &str = "/mnt/e/Projects/project-github/srs-exporter/config.toml";
const DEFAULT_SERVICE_NAME: &str = "srs";
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'{')
    .add(b'}')
    .add(b':')
    .add(b',');

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

pub fn parse_config() -> Result<SrsExporterConfig> {
    use std::env;
    let read_bytes = std::fs::read_to_string(CONFIG_LOCATION)?;
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

/**
 * did not process response yet
 */
pub async fn register_service(SrsExporterConfig { srs, nacos }: &SrsExporterConfig) -> Result<()> {
    let client = reqwest::Client::new();
    let body = client
            .post(format!(
                "http://{}:{}/nacos/v1/ns/instance?serviceName={}&ip={}&port={}&namespaceId={}&groupName={}",
                nacos.host,
                nacos.port,
                DEFAULT_SERVICE_NAME,
                srs.host,
                srs.rtmp_port,
                nacos.namespace_id,
                nacos.group_name
            ).as_str())
            .send()
            .await?
            .text()
            .await?;
    println!("注册返回 {:?}", body);
    Ok(())
}

pub async fn ping_pong(SrsExporterConfig { srs, nacos }: &SrsExporterConfig) -> Result<()> {
    let svc_name = format!("{}@@{}", nacos.group_name, DEFAULT_SERVICE_NAME);
    // TODO check SRS liveness
    let beat = format!(
        "{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}",
        svc_name, srs.host, srs.rtmp_port
    );
    let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();
    let client = reqwest::Client::new();
    let body = client
        .put(
            format!(
                "http://{}:{}/nacos/v1/ns/instance/beat?namespaceId={}&serviceName={}&beat={}",
                nacos.host,
                nacos.port,
                nacos.namespace_id,
                svc_name,
                encoded_beat
            )
            .as_str(),
        )
        .send()
        .await?
        .text()
        .await?;
    println!("心跳返回 {:?}", body);
    Ok(())
}

// pub fn register_service(SrsExporterConfig { srs, nacos }: &SrsExporterConfig) -> Result<()> {
//     let client = reqwest::blocking::Client::new();
//     client
//             .post(format!(
//                 "http://{}:{}/v1/ns/instance?serviceName={}&ip={}&port={}&namespaceId={}&groupName={}",
//                 nacos.host,
//                 nacos.port,
//                 DEFAULT_SERVICE_NAME,
//                 srs.host,
//                 srs.rtmp_port,
//                 nacos.namespace_id,
//                 nacos.group_name
//             ).as_str())
//             .send()
//             .unwrap();
//     Ok(())
// }

// pub fn ping_pong(SrsExporterConfig { srs, nacos }: &SrsExporterConfig) -> Result<()> {
//     // TODO check SRS liveness
//     let beat = format!(
//         "{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}",
//         DEFAULT_SERVICE_NAME, srs.host, srs.rtmp_port
//     );
//     let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();
//     let client = reqwest::blocking::Client::new();
//     client
//         .put(
//             format!(
//                 "http://{}:{}/v1/ns/instance/beat?serviceName={}&beat={}",
//                 nacos.host, nacos.port, DEFAULT_SERVICE_NAME, encoded_beat
//             )
//             .as_str(),
//         )
//         .send()
//         .unwrap();
//     Ok(())
// }

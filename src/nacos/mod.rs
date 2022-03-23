use crate::{AppError, SrsExporterConfig};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::Url;
use std::collections::HashMap;
use std::result::Result;

const DEFAULT_SERVICE_NAME: &str = "srs";
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'{')
    .add(b'}')
    .add(b':')
    .add(b',');

/**
 * Nacos Client
 */
#[derive(Clone, Debug)]
pub struct NacosClient {
    srs_exporter_config: SrsExporterConfig,
}

impl NacosClient {
    pub fn new(srs_exporter_config: &SrsExporterConfig) -> Self {
        Self {
            srs_exporter_config: srs_exporter_config.clone(),
        }
    }

    /**
     * register srs as a service in Nacos
     * add srs-exporter config in metadata, for nacos client able to fetch data from prometheus [instance]
     */
    pub async fn register_service(&self) -> Result<(), AppError> {
        let SrsExporterConfig { app, nacos, srs } = self.srs_exporter_config.clone();

        let metadata = HashMap::from([
            ("metric_host", app.host),
            ("metric_port", app.port.to_string()),
            ("metric_path", String::from("/metrics")),
        ]);
        let url = Url::parse_with_params(
            format!("http://{}:{}/nacos/api/ns/instance", nacos.host, nacos.port).as_str(),
            &[
                ("serviceName", DEFAULT_SERVICE_NAME.to_string()),
                ("ip", srs.host),
                ("port", srs.rtmp_port.to_string()),
                ("namespaceId", nacos.namespace_id),
                ("group", nacos.group_name),
                ("metadata", json::stringify(metadata)),
            ],
        )
        .unwrap();
        match reqwest::Client::new()
            .post(url)
            .header("Connection", "close")
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::NacosUnreachable),
        }
    }

    /**
     * use heart beat to keep srs service healthy
     */
    pub async fn ping_pong(&self) -> Result<(), AppError> {
        match self.check_srs_healthy().await {
            Ok(_) => {
                let SrsExporterConfig { app, nacos, srs } = self.srs_exporter_config.clone();
                let metadata = HashMap::from([
                    ("metric_host", app.host),
                    ("metric_port", app.port.to_string()),
                    ("metric_path", String::from("/metrics")),
                    ("srs_mode", srs.mode),
                ]);
                // combine group_name with service_name
                let svc_name = format!("{}@@{}", nacos.group_name, DEFAULT_SERVICE_NAME);
                let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{}}}", svc_name, srs.host, srs.rtmp_port, json::stringify(metadata));
                let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();
                let url = format!(
                    "http://{}:{}/nacos/v1/ns/instance/beat?namespaceId={}&serviceName={}&beat={}",
                    nacos.host, nacos.port, nacos.namespace_id, svc_name, encoded_beat
                );
                match reqwest::Client::new()
                    .put(url.as_str())
                    .header("Connection", "close")
                    .send()
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(AppError::NacosUnreachable),
                }
            }
            Err(e) => Err(e),
        }
    }

    /**
     * just check srs http api is ok or not
     */
    async fn check_srs_healthy(&self) -> Result<(), AppError> {
        let SrsExporterConfig {
            app: _,
            nacos: _,
            srs,
        } = self.srs_exporter_config.clone();
        let url = format!("http://{}:{}/api/v1/summaries", srs.host, srs.http_port);

        match reqwest::Client::new()
            .get(url.as_str())
            .header("Connection", "close")
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::SrsUnreachable),
        }
    }
}

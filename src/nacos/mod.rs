use std::collections::HashMap;
use std::result::Result;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::Url;

use crate::{AppError, SrsExporterConfig};

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
        let SrsExporterConfig {
            app, nacos, srs, ..
        } = self.srs_exporter_config.clone();

        let metadata = HashMap::from([
            ("cluster_mode", srs.mode),
            ("intranet_host", srs.host),
            ("metric_host", app.host),
            ("metric_port", app.port.to_string()),
            ("metric_path", String::from("/metrics")),
        ]);
        let mut params = vec![
            ("serviceName", DEFAULT_SERVICE_NAME.to_string()),
            ("ip", srs.domain),
            ("port", srs.rtmp_port.to_string()),
            ("namespaceId", nacos.namespace_id),
            ("group", nacos.group_name),
            ("metadata", json::stringify(metadata)),
        ];
        // 如果Nacos开启了认证
        if nacos.auth {
            params.push(("username", nacos.username));
            params.push(("password", nacos.password));
        }
        let url = Url::parse_with_params(
            format!("http://{}:{}/nacos/api/ns/instance", nacos.host, nacos.port).as_str(),
            &params,
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
                let SrsExporterConfig {
                    app, nacos, srs, ..
                } = self.srs_exporter_config.clone();
                let metadata = HashMap::from([
                    // origin or edge
                    ("cluster_mode", srs.mode),
                    // srs host used inside servers
                    ("intranet_host", srs.host),
                    ("metric_host", app.host),
                    ("metric_port", app.port.to_string()),
                    ("metric_path", String::from("/metrics")),
                ]);
                // combine group_name with service_name
                let svc_name = format!("{}@@{}", nacos.group_name, DEFAULT_SERVICE_NAME);
                // srs domain for dispatching to internet users
                let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{}}}", svc_name, srs.domain, srs.rtmp_port, json::stringify(metadata));
                let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();
                let mut params = vec![
                    ("namespaceId", nacos.namespace_id),
                    ("serviceName", svc_name),
                    ("beat", encoded_beat),
                ];
                // 如果Nacos开启了认证
                if nacos.auth {
                    params.push(("username", nacos.username));
                    params.push(("password", nacos.password));
                }
                let url = Url::parse_with_params(
                    format!(
                        "http://{}:{}/nacos/v1/ns/instance/beat",
                        nacos.host, nacos.port
                    )
                    .as_str(),
                    &params,
                )
                .unwrap();

                match reqwest::Client::new()
                    .put(url)
                    // connection no further usage
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
            ..
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

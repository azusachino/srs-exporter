use crate::SrsExporterConfig;
use anyhow::Result;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

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
     */
    pub async fn register_service(&self) -> Result<()> {
        let SrsExporterConfig {
            port: _,
            nacos,
            srs,
        } = self.srs_exporter_config.clone();

        // just don't catch the response
        reqwest::Client::new()
            .post(format!(
                "http://{}:{}/nacos/v1/ns/instance?serviceName={}&ip={}&port={}&namespaceId={}&groupName={}",
                nacos.host,
                nacos.port.unwrap(),
                DEFAULT_SERVICE_NAME,
                srs.host,
                srs.rtmp_port.unwrap(),
                nacos.namespace_id,
                nacos.group_name
            ).as_str())
            .send()
            .await?;
        // .text()
        // .await?;
        // println!("服务注册 {:?}", body);
        Ok(())
    }

    /**
     * use heart beat to keep srs service healthy
     */
    pub async fn ping_pong(self) -> Result<()> {
        match self.check_srs_healthy().await {
            Ok(_) => {
                let SrsExporterConfig {
                    port: _,
                    nacos,
                    srs,
                } = self.srs_exporter_config.clone();
                // combine group_name with service_name
                let svc_name = format!("{}@@{}", nacos.group_name, DEFAULT_SERVICE_NAME);
                let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", svc_name, srs.host, srs.rtmp_port.unwrap());
                let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();

                reqwest::Client::new()
                    .put(
                        format!(
                            "http://{}:{}/nacos/v1/ns/instance/beat?namespaceId={}&serviceName={}&beat={}",
                             nacos.host, nacos.port.unwrap(), nacos.namespace_id, svc_name, encoded_beat
                        )
                        .as_str(),
                    )
                    .send()
                    .await?;
                //     .text()
                //     .await?;
                // println!("心跳 {:?}", body);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /**
     * just check srs http api is ok or not
     */
    async fn check_srs_healthy(&self) -> Result<bool> {
        let SrsExporterConfig {
            port: _,
            nacos: _,
            srs,
        } = self.srs_exporter_config.clone();
        reqwest::Client::new()
            .get(
                format!(
                    "http://{}:{}/api/v1/summaries",
                    srs.host,
                    srs.http_port.unwrap()
                )
                .as_str(),
            )
            .send()
            .await?;
        //     .text()
        //     .await?;
        // println!("SRS Summary {:?}", body);
        Ok(true)
    }
}

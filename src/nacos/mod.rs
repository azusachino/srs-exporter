// use crate::{NacosConfig, SrsConfig, SrsExporterConfig};
// use anyhow::Result;
// use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// const DEFAULT_SERVICE_NAME: &str = "srs";
// const FRAGMENT: &AsciiSet = &CONTROLS
//     .add(b' ')
//     .add(b'"')
//     .add(b'{')
//     .add(b'}')
//     .add(b':')
//     .add(b',');

// #[derive(Clone, Debug)]
// pub struct NacosClient<'a> {
//     nacos_config: &'a NacosConfig,
//     srs_config: &'a SrsConfig,
// }

// impl NacosClient {
//     pub fn new(srs_exporter_config: SrsExporterConfig) -> Self {
//         return Self {
//             nacos_config: srs_exporter_config.nacos,
//             srs_config: srs_exporter_config.srs,
//         };
//     }

//     /**
//      * did not process response yet
//      */
//     pub async fn register_service(self) -> Result<()> {
//         let client = reqwest::Client::new();
//         let body = client
//             .post(format!(
//                 "http://{}:{}/v1/ns/instance?serviceName={}&ip={}&port={}&namespaceId={}&groupName={}",
//                 self.nacos_config.host,
//                 self.nacos_config.port,
//                 DEFAULT_SERVICE_NAME,
//                 self.srs_config.host,
//                 self.srs_config.rtmp_port,
//                 self.nacos_config.namespace_id,
//                 self.nacos_config.group_name
//             ).as_str())
//             .send()
//             .await?
//             .text()
//             .await?;

//         Ok(())
//     }

//     pub async fn ping_pong(self) -> Result<()> {
//         let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", DEFAULT_SERVICE_NAME, self.srs_config.host, self.srs_config.rtmp_port);
//         let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();
//         let client = reqwest::Client::new();
//         client
//             .put(
//                 format!(
//                     "http://{}:{}/v1/ns/instance/beat?serviceName={}&beat={}",
//                     self.nacos_config.host,
//                     self.nacos_config.port,
//                     DEFAULT_SERVICE_NAME,
//                     encoded_beat
//                 )
//                 .as_str(),
//             )
//             .send()
//             .await?;
//         Ok(())
//     }
// }

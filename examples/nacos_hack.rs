use anyhow::Result;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'{')
    .add(b'}')
    .add(b':')
    .add(b',');

#[tokio::main]
async fn main() -> Result<()> {
    let nacos = "localhost:8848";
    let namespace = "public";

    const GROUP_NAME: &str = "public";
    const DEFAULT_SERVICE_NAME: &str = "my_svc";

    loop {
        // combine group_name with service_name
        let svc_name = format!("{}@@{}", GROUP_NAME, DEFAULT_SERVICE_NAME);
        // let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", svc_name, "abc", "1234");
        // Missing `}}` will cause nacos JacksonMalFormatException, and the nacos interface will act abnormal
        let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}", svc_name, "abc", "1234");
        let encoded_beat = utf8_percent_encode(&beat, FRAGMENT).to_string();

        reqwest::Client::new()
            .put(
                format!(
                    "http://{}/nacos/v1/ns/instance/beat?namespaceId={}&serviceName={}&beat={}",
                    nacos, namespace, svc_name, encoded_beat
                )
                .as_str(),
            )
            .send()
            .await?;
    }
}

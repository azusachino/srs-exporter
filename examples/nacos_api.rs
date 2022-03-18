// use anyhow::Result;
// use reqwest::{Client, ClientBuilder};

// const REG_URL: &str = "http://localhost:8848/nacos/v1/ns/instance?port=1935&healthy=true&ip=172.31.103.161&serviceName=srs&namespaceId=scv&groupName=scv";
// const BEAT_URL: &str = "http://localhost:8848/nacos/v1/ns/instance/beat?port=1935&healthy=true&ip=172.31.103.161&serviceName=srs&namespaceId=scv&beat={port=1935, ip='172.31.103.161', weight=0.0, serviceName='srs', cluster='null', metadata=null, scheduled=false, period=0, stopped=false}
// ";

fn main() {}

// #[tokio::main]
// async fn main() -> Result<()> {
//     let req_client: Client = ClientBuilder::new().build()?;

//     req_client
//         .execute(req_client.post(REG_URL).build().unwrap())
//         .await?;

//     let req_client_clone = req_client.clone();
//     tokio::spawn(async move {
//         loop {
//             req_client_clone
//                 .execute(req_client_clone.put(BEAT_URL).build().unwrap())
//                 .await
//                 .unwrap();
//             std::thread::sleep(std::time::Duration::from_secs(2));
//         }
//     });

//     std::thread::sleep(std::time::Duration::from_secs(120));
//     Ok(())
// }

// const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'{').add(b'}').add(b':').add(b',');
// ///
// /// https://nacos.io/
// /// http://127.0.0.1:8848/nacos/v1/ns/instance?serviceName=rust-microservice&ip=127.0.0.1&port=8080
// pub fn register_service() {
//     println!("register service: {:?}", NACOS_SERVER);

//     tokio::spawn(
//         async {
//             let client = reqwest::Client::new();
//             let body = client.post(
//                 format!("{}/v1/ns/instance?serviceName={}&ip={}&port={}",
//                         NACOS_SERVER,
//                         PROVIDER_NAME,
//                         PROVIDER_HOST,
//                         PROVIDER_PORT).as_str()
//             ).send().unwrap().text();
//             println!("{:?}", body);
//         }
//     );
// }
// fn ping() {
//     //
//     // nacos 文档中没有说明 metadata 必选, 测试发现，如果没有 metadata 信息， java 端会有错误
//     //
//     let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", PROVIDER_NAME, PROVIDER_HOST, PROVIDER_PORT);
//     let  encode = utf8_percent_encode(&beat, FRAGMENT).to_string();
//     task::spawn(
//         async move {

//             let client = reqwest::blocking::Client::new();
//             let _body = client.put(
//                 format!("{}/v1/ns/instance/beat?serviceName={}&beat={}",
//                         NACOS_SERVER,
//                         PROVIDER_NAME,
//                         encode
//                 ).as_str()
//             ).send().unwrap().text();
//             println!("ping result:{:?}", _body);
//         }
//     );
// }

// pub fn ping_schedule() {
//     println!("ping schedule");
//     loop {
//         ping();
//         std::thread::sleep(Duration::from_secs(1));
//     }
// }

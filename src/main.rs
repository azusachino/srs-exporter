use anyhow::Result;
use chrono::prelude::Local;
use prometheus::Registry;
use srs_exporter::{
    parse_config, NacosClient, SrsConfig, StreamCollector, CURRENT_VERSION, DEFAULT_CONFIG,
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() {
    // treat first arg as config file location
    let f = match std::env::args().nth(1) {
        Some(f) => f,
        None => DEFAULT_CONFIG.to_string(),
    };

    let toml_config = parse_config(f).unwrap();
    // container environment compatiable
    let addr = format!("0.0.0.0:{}", toml_config.port.unwrap());
    let listener = TcpListener::bind(addr.clone()).await.unwrap();

    println!(
        "Srs Exporter is listening {}, Current Version is {}",
        addr, CURRENT_VERSION
    );

    // spawn a task to check srs and report to nacos
    let config_clone = toml_config.clone();
    tokio::spawn(async move {
        let nacos_client = NacosClient::new(&config_clone);
        nacos_client.register_service().await.unwrap();
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // process every two seconds
            nacos_client.clone().ping_pong().await.unwrap();
        }
    });

    let srs: SrsConfig = toml_config.srs;
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let fake_content = collect_metrics(&srs).await.unwrap();
        let current = Local::now().to_rfc2822();
        // let current = "Thu, 03 Mar 2022 08:34:52 GMT";
        // Important!!! HTTP HEADER要顶格写
        let fake_header = format!(
            "
HTTP/1.1 200 OK
Content-Type: text/plain; version=0.0.4; charset=utf-8
Content-Length: {}
Date: {}
",
            fake_content.as_bytes().len(),
            current,
        );
        let res = format!("{}\n{}", fake_header, fake_content);
        socket.write_all(res.as_bytes()).await.unwrap();
    }
}

async fn collect_metrics(srs_config: &SrsConfig) -> Result<String> {
    let r = Registry::new();
    let su = StreamCollector::new(r, srs_config);
    Ok(su.collect().await?)
}

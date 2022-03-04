use anyhow::Result;
use chrono::prelude::Local;
use prometheus::Registry;
use srs_exporter::{
    parse_config, ping_pong, register_service, SrsConfig, StreamUsage, CURRENT_VERSION,
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() {
    let toml_config = parse_config().unwrap();
    let addr = "127.0.0.1:9007";
    let listener = TcpListener::bind(addr.clone()).await.unwrap();

    println!(
        "Srs Exporter is listening {}, Current Version is {}",
        addr, CURRENT_VERSION
    );

    // spawn a task to check srs and report to nacos
    let config_clone = toml_config.clone();
    // std::thread::spawn(move || {
    //     register_service(&config_clone).unwrap();
    //     loop {
    //         std::thread::sleep(std::time::Duration::from_secs(2));
    //         // process every two seconds
    //         ping_pong(&config_clone).unwrap();
    //     }
    // });
    tokio::spawn(async move {
        register_service(&config_clone).await.unwrap();
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // process every two seconds
            ping_pong(&config_clone).await.unwrap();
        }
    });

    let srs: SrsConfig = toml_config.srs;
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let fake_html_content = collect_metrics(&srs).await.unwrap();
        // let fake_html_content = "Hello World";
        let fake_html = format!(
            "<html>
              <header>
                <title>SRS Metrics</title>
              </header>
              <body>
                <pre style=\"word-wrap: break-word; white-space: pre-wrap\">
{}
                </pre>
              </body>
            </html>",
            fake_html_content
        );
        let current = Local::now().to_rfc2822();
        // let current = "Thu, 03 Mar 2022 08:34:52 GMT";
        // Important!!! HTTP HEADER要顶格写
        let fake_header = format!(
            "
HTTP/1.1 200 OK
Server: nginx/1.16.1
Date: {}
Content-Type: text/html
Content-Length: {}
Accept-Ranges: bytes
",
            current,
            fake_html.as_bytes().len(),
        );
        let res = format!("{}\n{}", fake_header, fake_html);
        socket.write_all(res.as_bytes()).await.unwrap();
    }
}

async fn collect_metrics(srs_config: &SrsConfig) -> Result<String> {
    let r = Registry::new();
    let su = StreamUsage::new(r, srs_config);
    Ok(su.collect().await?)
}

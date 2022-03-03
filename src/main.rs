use anyhow::Result;
use chrono::prelude::Local;
use prometheus::Registry;
use srs_exporter::{Collector, StreamUsage};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9007";
    let listener = TcpListener::bind(addr.clone()).await.unwrap();

    println!("Server is listening {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let fake_html_content = collect_metrics().await.unwrap();
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

async fn collect_metrics() -> Result<String> {
    let r = Registry::new();
    let su = StreamUsage::new(r);
    Ok(su.collect().unwrap())
}

use reqwest::Url;

fn main() {
    // let map = std::collections::HashMap::from([("abc", "1"), ("abcd", "2")]);
    let metadata = std::collections::HashMap::from([
        ("metric_host", "app.host"),
        ("metric_port", "app.port.unwrap().to_string()"),
        ("metric_path", "/metrics"),
    ]);

    let url = format!(
        "http://localhost:8080?query={}&metadata={}",
        "oo",
        json::stringify(metadata)
    );

    println!("{:?}", Url::parse(url.as_str()).unwrap());
}

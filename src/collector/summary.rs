use crate::SrsConfig;
use prometheus::{Gauge, Opts, Registry};

const BASE_URL: &str = "/api/v1/summaries/";

#[derive(Clone, Debug)]
pub struct SummaryCollector {
    srs_url: String,
    cpu_percent: Gauge,
}

impl SummaryCollector {
    pub fn new(registry: &Registry, srs_config: &SrsConfig) -> Self {
        let sc = Self {
            srs_url: format!(
                "http://{}:{}{}",
                srs_config.host,
                srs_config.http_port.unwrap(),
                BASE_URL
            ),
            cpu_percent: Gauge::with_opts(Opts::new("srs_cpu_percent", "Cpu usage percent of srs"))
                .unwrap(),
        };
        registry.register(Box::new(sc.cpu_percent.clone())).unwrap();
        sc
    }

    pub async fn collect(self) {
        // get current stream usage
        let ret = reqwest::Client::new()
            .get(self.srs_url)
            .send()
            .await
            .unwrap()
            .text()
            // .text()
            .await
            .unwrap();
        println!("Summary Response {}", ret);
        self.cpu_percent.inc();
    }
}

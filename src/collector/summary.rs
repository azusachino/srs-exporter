use crate::{AppError, SrsConfig};
use prometheus::{Gauge, Opts, Registry};
use serde_derive::Deserialize;
use std::result::Result;

const BASE_URL: &str = "/api/v1/summaries";

#[derive(Clone, Debug)]
pub struct SummaryCollector {
    srs_url: String,
    mem_percent: Gauge,
    cpu_percent: Gauge,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
struct SummaryResponse {
    code: i16,
    server: String,
    data: SummaryData,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
struct SummaryData {
    ok: bool,
    now_ms: u64,
    /**
     * We only care about Self Status
     */
    #[serde(alias = "self")]
    status: SelfStatus,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
struct SelfStatus {
    mem_percent: f64,
    cpu_percent: f64,
}

impl SummaryCollector {
    /**
     * Constructor
     */
    pub fn new(registry: &Registry, srs_config: &SrsConfig) -> Self {
        let sc = Self {
            srs_url: format!(
                "http://{}:{}{}",
                srs_config.host, srs_config.http_port, BASE_URL
            ),
            mem_percent: Gauge::with_opts(Opts::new(
                "srs_mem_percent",
                "Memory usage percent of SRS",
            ))
            .unwrap(),
            cpu_percent: Gauge::with_opts(Opts::new("srs_cpu_percent", "Cpu usage percent of SRS"))
                .unwrap(),
        };
        registry.register(Box::new(sc.mem_percent.clone())).unwrap();
        registry.register(Box::new(sc.cpu_percent.clone())).unwrap();
        sc
    }

    /**
     * Collect Cpu/Mem status
     */
    pub async fn collect(&self) -> Result<(), AppError> {
        match reqwest::Client::new()
            .get(self.srs_url.clone())
            .header("Connection", "close")
            .send()
            .await
        {
            Ok(res) => match res.json::<SummaryResponse>().await {
                Ok(ret) => {
                    let SummaryResponse {
                        code: _,
                        server: _,
                        data:
                            SummaryData {
                                ok: _,
                                now_ms: _,
                                status,
                            },
                    } = ret;
                    self.mem_percent.set(status.mem_percent);
                    self.cpu_percent.set(status.cpu_percent);
                    Ok(())
                }
                Err(_) => Err(AppError::SrsUnreachable),
            },
            Err(_) => Err(AppError::SrsUnreachable),
        }
    }
}

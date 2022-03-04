#![allow(unused)]

use anyhow::Result;
use reqwest::{self, get};
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct SummaryResponse {
    code: i8,
    server: String,
    data: SummaryData,
}

#[derive(Debug, Deserialize)]
struct SummaryData {
    ok: bool,
    now_ms: u64,
    // sel: SelfData,
    system: SystemData,
}

#[derive(Debug, Deserialize)]
struct SelfData {
    version: String,
    pid: u32,
    ppid: u32,
    argv: String,
    cwd: String,
}

#[derive(Debug, Deserialize)]
struct SystemData {
    cpu_percent: f32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:1985/api/v1/summaries";
    let ret = get(url).await?.json::<SummaryResponse>().await?;
    println!("{:?}", ret);
    Ok(())
}

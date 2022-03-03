use anyhow::Result;
use reqwest::{self, get};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:1985/api/v1/clients/";
    let ret = get(url).await?.json::<Value>().await?;
    println!("{:?}", ret);
    Ok(())
}

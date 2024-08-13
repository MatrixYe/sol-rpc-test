use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

use crate::cli::Cli;

mod cli;
mod rpc;
mod methods;

#[derive(Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
struct RpcResponse {
    jsonrpc: String,
    id: u32,
    error: Option<RpcError>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    let count = args.count;
    let rpc_url = args.url;
    println!("Count = {:?}", count);
    println!("rpc_url = {:?}", rpc_url);

    match test_http_request(rpc_url.as_str()).await {
        Ok(duration) => {
            println!("Request succeeded in {:?} seconds.", duration);
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }
}


async fn test_http_request(url: &str) -> Result<f64> {
    let client = Client::new();
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
            { "encoding": "base58" }
        ]
    });

    // 开始计时
    let start = Instant::now();

    let response = client.post(url)
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("Failed to send request")?;

    // 计算耗时
    let duration = start.elapsed().as_secs_f64();

    if response.status().is_success() {
        let rpc_response: RpcResponse = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        // 检查是否存在 error 字段
        if let Some(error) = rpc_response.error {
            return Err(anyhow::anyhow!(
                "RPC error occurred: {} (code: {})",
                error.message,
                error.code
            ));
        }
        Ok(duration)
    } else {
        Err(anyhow::anyhow!("Request failed with status: {}", response.status()))
    }
}
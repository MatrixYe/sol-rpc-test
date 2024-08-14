use std::time::Instant;

use anyhow::{Context, Result};
use clap::builder::Str;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::methods::{JsonRpcInterface, RpcMethod};

/// @Name rpc
///
/// @Date 2024/8/13 下午5:35
///
/// @Author Matrix.Ye
///
/// @Description:
#[derive(Deserialize, Debug)]
pub struct RpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>, // 新增的 result 字段，类型为 Option<Value>    id: u32,
    pub error: Option<RpcError>,
    pub id: u8,
}

#[derive(Deserialize, Debug)]
pub struct Outcome {
    pub name: String,
    pub is_ok: bool,
    pub resp: Option<RpcResponse>,
    pub elapsed: f64,
    pub err: Option<String>,
}


async fn request(url: String, method: RpcMethod) -> Result<RpcResponse> {
    println!("------------------------- Test: {:?} -------------------------", method.method_name());
    let client = Client::new();
    let response = client.post(url)
        .json(&method.request_body())
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("Failed to send request")?;
    if response.status().is_success() {
        let rpc_response: RpcResponse = response.json().await.context("Failed to parse JSON response")?;
        println!("Result:{:?}", rpc_response.result);
        // 检查是否存在 error 字段
        if let Some(error) = rpc_response.error {
            println!("Error: {:?}", error);
            return Err(anyhow::anyhow!(
                "RPC error msg: {} (code: {})",
                error.message,
                error.code
            ));
        }
        Ok(rpc_response)
    } else {
        println!("Request failed with status: {}", response.status());
        Err(anyhow::anyhow!("Request failed with status: {}", response.status()))
    }
}

pub async fn send_rpc_request(url: String, method: RpcMethod) -> Outcome {

    // 开始计时
    let start = Instant::now();
    let res = request(url, method.clone()).await;
    // 计算耗时
    let duration = start.elapsed().as_secs_f64();
    match res {
        Ok(resp) => {
            Outcome {
                name: method.method_name().to_string(),
                is_ok: true,
                resp: Some(resp),
                elapsed: duration,
                err: None,
            }
        }
        Err(e) => {
            Outcome {
                name: method.method_name().to_string(),
                is_ok: false,
                resp: None,
                elapsed: duration,
                err: Some(e.to_string()),
            }
        }
    }
}

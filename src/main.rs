use std::fs::File;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use strum::IntoEnumIterator;

use crate::cli::Cli;
use crate::methods::{JsonRpcInterface, RpcMethod};
use crate::rpc::{Outcome, send_rpc_request};

mod cli;
mod methods;
mod rpc;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    let outfile = args.out;
    let url = args.url;
    println!("outfile = {:?}", outfile);
    println!("url = {:?}", url);
    // let url = "https://api.devnet.solana.com";
    // let url = "https://api.mainnet-beta.solana.com";
    // let url = "http://64.130.58.16:8899";
    let mut buff: Vec<Outcome> = vec![];
    // 遍历全部rpc请求
    // let mut temp = vec![];
    // temp.push(RpcMethod::GetSlotLeaders);
    // temp.push(RpcMethod::GetProgramAccounts);
    // temp.push(RpcMethod::GetTokenAccountBalance);
    // // temp.push(RpcMethod::GetTransaction);
    // for method in temp {
    //     let outcome = send_rpc_request(url, method).await;
    //     buff.push(outcome);
    //     tokio::time::sleep(Duration::from_secs(2)).await;
    // }

    for method in RpcMethod::iter() {
        let outcome = send_rpc_request(url.clone(), method).await;
        buff.push(outcome);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }


    println!("------------------------- All RPC Method Test Completed. -------------------------", );
    // 处理测试结果
    for outcome in &buff {
        println!("Method:{:?} | OK:{:?} | Elapsed:{:?}s | Err:{:?}", outcome.name, outcome.is_ok, outcome.elapsed, outcome.err);
    }
    let file_path = outfile.as_str();
    let mut wtr = csv::Writer::from_writer(File::create(file_path).expect(""));
    wtr.write_record(&["url", "method_name", "is_ok", "elapsed(s)", "err"]).expect("write_record failed");
    for outcome in buff {
        wtr.write_record(&[
            &url,
            &outcome.name,
            &outcome.is_ok.to_string(),
            &outcome.elapsed.to_string(),
            &outcome.err.clone().unwrap_or_default(),
        ]).expect("write_record failed");
    }

    wtr.flush().expect("flush failed");
    println!("Data written to {}", file_path);
}


use anyhow::Context;
use clap::Parser;
use strum::IntoEnumIterator;

use crate::cli::Cli;
use crate::methods::JsonRpcInterface;

mod cli;
mod methods;
mod rpc;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    let count = args.count;
    let rpc_url = args.url;
    println!("Count = {:?}", count);
    println!("rpc_url = {:?}", rpc_url);
    let url = "https://api.devnet.solana.com";
    // let url = "https://api.mainnet-beta.solana.com";
    // let url = "http://64.130.58.16:8899";

    // for method in RpcMethod::iter() {
    //     let outcome = send_rpc_request(url, RpcMethod::GetSlot).await;
    // println!("Method:{:?} OK:{:?} Elapsed:{:?} Resp:{:?} Err:{:?}", method, outcome.is_ok, outcome.elapsed, outcome.resp, outcome.err);
    // println!("Method:{:?} | OK:{:?} | Elapsed:{:?} | Err:{:?}", method, outcome.is_ok, outcome.elapsed, outcome.err);
    // tokio::time::sleep(Duration::from_secs(2)).await;
    // }
}


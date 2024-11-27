use std::sync::Arc;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};
use starknet::providers::Provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let url = Url::parse("http://localhost:5050").unwrap();
    let provider = Arc::new(JsonRpcClient::new(HttpTransport::new(url)));
    println!("{:?}", provider);
    let x = provider.block_hash_and_number().await?;
    println!("{:?}", x);
    Ok(())
}
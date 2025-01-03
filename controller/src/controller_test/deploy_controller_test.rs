use account_sdk::signers::{Owner, Signer};
use starknet::core::utils::cairo_short_string_to_felt;
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider, Url};
use starknet::signers::SigningKey;

pub(crate) async fn test_deploy_controller() {
    // Create signers
    let owner = Owner::Signer(Signer::Starknet(SigningKey::from_secret_scalar(felt!(
        "0xc5b2fcab997346f3ea1c00b002ecf6f382c5f9c9659a3894eb783c5320f912"
    ))));
    //dbg!(owner);
    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("http://127.0.0.1:5050").unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();

    // Create a new Controller instance
    let username = "testuser".to_string();
    let salt = cairo_short_string_to_felt(&username).unwrap();
}
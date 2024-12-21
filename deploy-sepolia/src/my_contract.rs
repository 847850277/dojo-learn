use std::sync::Arc;
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::core::types::contract::SierraClass;
use starknet::macros::{felt, selector};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;
use crate::startknet_contract::casm_class_hash;

pub(crate) async fn decare_my_contract() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_actions.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf"),
    ));
    let address = felt!("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let flattened_class = class.flatten().unwrap();

    let account = account.declare_v2(Arc::new(flattened_class), casm_class_hash);

    let fee_response = account.estimate_fee().await;

    let deploy_response = account.send().await;
    dbg!(deploy_response);
}


// 调用对应合约的方法
pub(crate) async fn call_spawn() {

    let contract_address =
        Felt::from_hex("0x009486b97b51beb5d31909b2e07bcadce5edf3d248f39b88f627fb5a78337eb5").unwrap();


    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let private_key = "0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";
    let address = "0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(private_key).unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    let selector = felt!("0x0217c73ea9ef26581623f20edd45571c1d024612b70d0af3e0842c5b0dc253cd");

    let transfer_response = account.execute_v1(vec![Call {
        to: contract_address,
        selector: selector!("spawn"),
        //selector: selector,
        calldata: vec![],
    }])//.max_fee(FieldElement::from_dec_str("1000000000000000000").unwrap())
        .send()
        .await;

    dbg!(transfer_response);

}
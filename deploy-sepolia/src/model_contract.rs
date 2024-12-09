use std::sync::Arc;
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::contract::ContractFactory;
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::types::contract::SierraClass;
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;
use crate::startknet_contract::class_hash;

pub(crate) async fn decare_position() {
    // 0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_m_Position.contract_class.json";

    let casm_class_hash = class_hash(path).await;

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

// 部署合约的地址0x00bf18649ed353a45317a2e2bc84431d92810e59cb83d487008ca9c7bc7ff57d
pub(crate) async fn deploy_position() {
    let decare_class_hash =
        felt!("0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
    ));

    let chain_id = provider.chain_id().await.unwrap();

    //let private_key = "0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";
    //let address = "0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be(private_key).unwrap(),
        felt!("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf"),
    ));
    //let address = FieldElement::from_hex_be(address).unwrap();
    let address = felt!("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a");

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(decare_class_hash, account);

    let owner = address.clone();
    let fee_recipient = address.clone();

    let deploy_result = contract_factory
        .deploy_v1(vec![], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");

    println!("{:?}", deploy_result);
}
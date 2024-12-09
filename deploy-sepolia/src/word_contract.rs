use crate::startknet_contract::class_hash;
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::contract::ContractFactory;
use starknet::core::chain_id;
use starknet::core::types::contract::SierraClass;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use std::sync::Arc;
use starknet::core::utils::get_selector_from_name;
use url::Url;
use crate::byte_array::{byte_array, byte_array_str};

pub async fn decare() {
    let path = "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";

    let casm_class_hash = class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        //Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
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


// 部署世界合约，初始化参数为世界合约的class_hash
async fn deplpy() {
    let decare_class_hash =
        felt!("0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6");

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

    let word_class_hash =
        felt!("0x149397d4ee0e1895700b62c74d73a1314f53924b1f76d9cd2cf4d865c18abd0");
    let deploy_result = contract_factory
        .deploy_v1(vec![word_class_hash], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");

    println!("{:?}", deploy_result);
}


// 注册命名空间,命名空间的含义为，当前世界在那个空间下,cairo参数类型为ByteArray
// 测试空间地址为0x123456
pub(crate) async fn register_namespace() {


    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
    ));
    let address = Felt::from_hex("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();
    let word_contract_address =
        Felt::from_hex("0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));


    //betsy
    let name_space_data = vec![
        //Felt::ZERO,
        Felt::from_hex("0x00").unwrap(),
        Felt::from_hex("0x6265747379").unwrap(),
        Felt::from_hex("0x05").unwrap(),
    ];

    let name_space_data = byte_array().await;

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_namespace").unwrap(),
            calldata: name_space_data,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);

}

// 注册事件
// 命名空间和 event_合约的class_hash
pub(crate) async fn register_event() {
    todo!()
}


// 注册model
// 命名空间 和 model_合约的class_hash
pub(crate) async fn register_model_position() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
    ));
    let address = Felt::from_hex("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();
    let word_contract_address =
        Felt::from_hex("0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));



    // a
    let name_space_data = byte_array().await;

    // model class_hash
    let class_hash = felt!("0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594");

    // name_space_data add class
    let mut calldata = name_space_data;
    calldata.push(class_hash);

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_model").unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}

pub(crate) async fn register_namespace_b() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
    ));
    let address = Felt::from_hex("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();
    let word_contract_address =
        Felt::from_hex("0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));


    let name_space_data = byte_array_str("b").await;

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_namespace").unwrap(),
            calldata: name_space_data,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}

pub(crate) async fn register_model_position_b() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
    ));
    let address = Felt::from_hex("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();
    let word_contract_address =
        Felt::from_hex("0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));



    // a
    let name_space_data = byte_array_str("b").await;

    // model class_hash
    let class_hash = felt!("0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594");

    // name_space_data add class
    let mut calldata = name_space_data;
    calldata.push(class_hash);

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_model").unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
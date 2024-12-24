use std::sync::Arc;
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::contract::ContractFactory;
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::core::types::contract::SierraClass;
use starknet::core::utils::get_selector_from_name;
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;
use crate::byte_array::byte_array_str;
use crate::startknet_contract::casm_class_hash;


// 账户2声明世界合约
pub(crate) async fn decare() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_world.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;
    println!("casm_class_hash: {:?}", casm_class_hash.to_hex_string());

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        //Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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


// 账号2部署世界合约
pub(crate) async fn deploy() {
    let decared_class_hash =
        felt!("0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
    ));

    let chain_id = provider.chain_id().await.unwrap();

    //let private_key = "0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";
    //let address = "0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be(private_key).unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    //let address = FieldElement::from_hex_be(address).unwrap();
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De");

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(decared_class_hash, account);

    let owner = address.clone();
    let fee_recipient = address.clone();

    let casm_word_class_hash =
        felt!("0x149397d4ee0e1895700b62c74d73a1314f53924b1f76d9cd2cf4d865c18abd0");
    let deploy_result = contract_factory
        .deploy_v1(vec![casm_word_class_hash], felt!("1234"), false)
        .send()
        .await
        .expect("Unable to deploy contract");

    println!("{:?}", deploy_result);
}

// 账号2注册命名空间
pub(crate) async fn register_namespace() {

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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


    let name_space_data = byte_array_str("dojo_starter").await;

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


// 账户2注册模型
pub(crate) async fn register_model_decare_move() {

    // 0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_m_Moves.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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

pub(crate) async fn register_model_move() {

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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



    // dojo_starter namespace
    let name_space_data = byte_array_str("dojo_starter").await;

    // model class_hash,从model的decalre中获取
    let class_hash = Felt::from_hex("0x070edf8f3be0b118e78f856f3ea9ebb652cba3684abaf7f299bfa6f93bf907c9").unwrap();
    // print class_hash hex_to_string
    println!("class_hash: {}", class_hash.to_hex_string());
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

pub(crate) async fn register_model_decare_position() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_m_Position.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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

pub(crate) async fn register_model_position() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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



    // dojo_starter namespace
    let name_space_data = byte_array_str("dojo_starter").await;

    // model class_hash,从model的decalre中获取
    let class_hash = Felt::from_hex("0x02283c68ecba5c60bbbbd3b00659808a02244468e41a1d2cdba1312d65b83594").unwrap();
    // print class_hash hex_to_string
    println!("class_hash: {}", class_hash.to_hex_string());
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

pub(crate) async fn register_model_decare_DirectionsAvailable() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_m_DirectionsAvailable.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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

pub(crate) async fn register_model_DirectionsAvailable() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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



    // dojo_starter namespace
    let name_space_data = byte_array_str("dojo_starter").await;

    // model class_hash,从model的decalre中获取
    let class_hash = Felt::from_hex("0x07deb48ccf95cc441a0489cfefdae54aeb6f8ec462ba13ff25e23f080e66cc2f").unwrap();
    // print class_hash hex_to_string
    println!("class_hash: {}", class_hash.to_hex_string());
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

pub(crate) async fn register_event_decare_move() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_e_Moved.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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

pub(crate) async fn register_event_move() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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

    // dojo_starter namespace
    let name_space_data = byte_array_str("dojo_starter").await;

    // model class_hash,从model的decalre中获取
    let class_hash = Felt::from_hex("0x05be0a05a5df3bd3b4fc17f8b1feb395cb463ced20ea41d4fbb9b86a4d7efc66").unwrap();
    // print class_hash hex_to_string
    println!("class_hash: {}", class_hash.to_hex_string());
    // name_space_data add class
    let mut calldata = name_space_data;
    calldata.push(class_hash);

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_event").unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}

pub(crate) async fn register_contract_decare_actions() {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/dev/dojo_starter_actions.contract_class.json";

    let casm_class_hash = casm_class_hash(path).await;

    let class: SierraClass = serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe"),
    ));
    let address = felt!("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De"); //FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

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

pub(crate) async fn register_contract_actions() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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

    //
    let salt = Felt::from_hex("0x123456").unwrap();
    let mut calldata = vec![salt];


    // dojo_starter namespace
    let mut name_space_data = byte_array_str("dojo_starter").await;

    // call append namespace_data
    calldata.append(&mut name_space_data);

    // contract class_hash
    let class_hash = felt!("0x0135fe43ec9004617b2a9fa5c028b02cca0a831a6d71755e207b76b4d01dcad2");

    // name_space_data add class
    calldata.push(class_hash);

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: get_selector_from_name("register_contract").unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
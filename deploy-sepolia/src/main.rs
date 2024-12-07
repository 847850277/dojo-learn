use starknet::core::types::contract::{CompiledClass, SierraClass};
use std::sync::Arc;

use cairo_starknet_2_7_1::{
    casm_contract_class::CasmContractClass as Cairo271CasmClass,
    contract_class::ContractClass as Cairo271Class,
};
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::contract::ContractFactory;
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Felt};
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    //declare_cairo1_contract().await;
    // let path =  "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";
    // //let path =  "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";
    // let class_hash = test_3(path).await;
    // println!("{}", class_hash);
    // let path = "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";
    // let class_hash = class_hash(path).await;
    // let string = class_hash.to_hex_string();
    // println!("{}", string);
    //println!("{}", class_hash);
    // hex print

    //decare().await;

    deplpy().await;

}

async fn deplpy() {

    let decare_class_hash = felt!("0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
    ));

    let chain_id = provider.chain_id().await.unwrap();

    //let private_key = "0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";
    //let address = "0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be(private_key).unwrap(),
        felt!("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf")
    ));
    //let address = FieldElement::from_hex_be(address).unwrap();
    let address = felt!("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a");

    let mut account = SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);


    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(decare_class_hash, account);

    let owner = address.clone();
    let fee_recipient = address.clone();

    let word_class_hash = felt!("0x149397d4ee0e1895700b62c74d73a1314f53924b1f76d9cd2cf4d865c18abd0");
        //FieldElement::from_hex_be("0x123456").unwrap();

    // let symbol =
    //     FieldElement::from_hex_be("0x123456").unwrap();

    let deploy_result = contract_factory
        .deploy_v1(vec![word_class_hash,
                        //symbol
        ], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");

    println!("{:?}", deploy_result);
}

pub async fn decare() {

    let path = "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";

    let casm_class_hash = class_hash(path).await;

    let class: SierraClass =
        serde_json::from_reader(std::fs::File::open(path).unwrap())
            .unwrap();


    println!("{}", casm_class_hash);


    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
        //Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));


    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //FieldElement::from_hex_be("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf").unwrap(),
        felt!("0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf")
    ));
    let address = felt!("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a");//FieldElement::from_hex_be("0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // We need to flatten the ABI into a string first
    let flattened_class = class.flatten().unwrap();

    let account = account.declare_v2(Arc::new(flattened_class), casm_class_hash);

    let fee_response = account.estimate_fee().await;
    dbg!(fee_response);

    let deploy_response = account.send().await;
    dbg!(deploy_response);


}


// pub async fn declare_cairo1_contract() {
//     // Sierra class artifact. Output of the `starknet-compile` command
//     let class: SierraClass =
//         serde_json::from_reader(std::fs::File::open("/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json").unwrap())
//             .unwrap();
//
//     let class_hash = class.class_hash().unwrap();
//     println!("{}", class_hash);
// }


pub(crate) async fn class_hash(path: &str) -> Felt {
    //let path = "/Users/zhengpeng/Work/Source/Code/rust-code/starknet-rs-learn/contract/_01_hello_world/target/dev/_01_hello_world_Balance.contract_class.json";
    let class: starknet::core::types::contract::SierraClass =
        serde_json::from_reader(std::fs::File::open(path).unwrap())
            .unwrap();

    // println!("1");
    //
    // let class_hash = class.class_hash();
    // println!("{:?}", class_hash);
    //
    // let result = class.flatten();
    // println!("{:?}", result);

    let sierra_class_json = serde_json::to_string(&class).unwrap();
    let contract_class: Cairo271Class = serde_json::from_str(&sierra_class_json).unwrap();

    // TODO: implement the `validate_compatible_sierra_version` call

    let casm_contract = Cairo271CasmClass::from_contract_class(
        contract_class,
        false,
        MAX_BYTECODE_SIZE_1,
    ).unwrap();

    let casm_class_json = serde_json::to_string(&casm_contract).unwrap();

    //println!("{}", casm_class_hash);
    let casm_class = serde_json::from_str::<CompiledClass>(&casm_class_json).unwrap();

    let casm_class_hash = casm_class.class_hash().unwrap();

    //Ok(casm_class_hash)
    //println!("{}", casm_class_hash);
    return casm_class_hash;

}
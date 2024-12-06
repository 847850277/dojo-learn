use starknet::core::types::contract::{CompiledClass, SierraClass};

use cairo_starknet_2_6_2::{
    casm_contract_class::CasmContractClass as Cairo262CasmClass,
    contract_class::ContractClass as Cairo262Class,
};

use cairo_starknet_2_5_4::{
    casm_contract_class::CasmContractClass as Cairo253CasmClass,
    contract_class::ContractClass as Cairo253Class,
};

use cairo_starknet_2_7_1::{
    casm_contract_class::CasmContractClass as Cairo271CasmClass,
    contract_class::ContractClass as Cairo271Class,
};

use starknet::core::types::FieldElement;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    //declare_cairo1_contract().await;
    let path =  "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json";
    let class_hash = test_3(path).await;
    println!("{}", class_hash);
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


pub(crate) async fn test_3(path: &str) -> FieldElement {
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
    println!("{}", casm_class_hash);
    return casm_class_hash;

}
use starknet::core::types::contract::{CompiledClass, SierraClass};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    declare_cairo1_contract().await;
}


pub async fn declare_cairo1_contract() {
    // Sierra class artifact. Output of the `starknet-compile` command
    let class: SierraClass =
        serde_json::from_reader(std::fs::File::open("/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/dojo-starter/target/release/dojo_starter_world.contract_class.json").unwrap())
            .unwrap();

    let class_hash = class.class_hash().unwrap();
    println!("{}", class_hash);
}
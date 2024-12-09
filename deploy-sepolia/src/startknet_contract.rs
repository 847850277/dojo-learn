use crate::MAX_BYTECODE_SIZE_1;
use starknet::core::types::contract::CompiledClass;
use starknet::core::types::Felt;

use cairo_starknet_2_7_1::{
    casm_contract_class::CasmContractClass as Cairo271CasmClass,
    contract_class::ContractClass as Cairo271Class,
};

pub(crate) async fn casm_class_hash(path: &str) -> Felt {
    let class: starknet::core::types::contract::SierraClass =
        serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
    let sierra_class_json = serde_json::to_string(&class).unwrap();
    let contract_class: Cairo271Class = serde_json::from_str(&sierra_class_json).unwrap();
    let casm_contract =
        Cairo271CasmClass::from_contract_class(contract_class, false, MAX_BYTECODE_SIZE_1).unwrap();
    let casm_class_json = serde_json::to_string(&casm_contract).unwrap();
    let casm_class = serde_json::from_str::<CompiledClass>(&casm_class_json).unwrap();
    casm_class.class_hash().unwrap()
}

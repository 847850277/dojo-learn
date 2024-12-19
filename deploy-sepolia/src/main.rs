pub mod startknet_contract;
pub mod word_contract;
mod byte_array;
mod model_contract;
mod event_contract;
mod my_contract;
mod call;

use starknet::accounts::Account;
use starknet::providers::Provider;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {

    word_test().await;
    //contract_call_test().await;

}

pub async fn word_test() {
    //word_contract::register_namespace().await;
    //word_contract::register_event().await;
    //model_contract::deploy_position().await;

    //word_contract::register_model_position().await;
    //word_contract::register_namespace_b().await;
    //word_contract::register_model_position_b().await;
    //                    execution_error: "Class with hash 0x070edf8f3be0b118e78f856f3ea9ebb652cba3684abaf7f299bfa6f93bf907c9 is already declared.",
    //model_contract::decare_move().await;
    //word_contract::register_model_move().await;
    //0x70edf8f3be0b118e78f856f3ea9ebb652cba3684abaf7f299bfa6f93bf907c9
    // model_contract::decare_direction().await;
    // word_contract::register_model_direction().await;
    //event_contract::decare_event().await;
    //word_contract::register_event().await;
    //my_contract::decare_my_contract().await;
    //word_contract::register_contract().await;

    //word_contract::init_contract().await;
    word_contract::permissions().await;
}

pub async  fn contract_call_test() {
    //println!("contract_call_test");

    my_contract::call_spawn().await;
}

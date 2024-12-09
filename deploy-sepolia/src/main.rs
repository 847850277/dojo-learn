pub mod startknet_contract;
pub mod word_contract;
mod byte_array;
mod model_contract;

use starknet::accounts::Account;
use starknet::providers::Provider;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {

    //word_contract::register_namespace().await;
    //word_contract::register_event().await;
    //model_contract::deploy_position().await;

    //word_contract::register_model_position().await;
    //word_contract::register_namespace_b().await;
    word_contract::register_model_position_b().await;
}

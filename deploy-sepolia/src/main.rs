pub mod startknet_contract;
pub mod word_contract;

use starknet::accounts::Account;
use starknet::providers::Provider;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {

    word_contract::register_namespace().await;
}

use starknet::accounts::Account;
use starknet::providers::Provider;

pub mod startknet_contract;
pub mod word_contract;
mod byte_array;
mod model_contract;
mod event_contract;
mod my_contract;
mod call;
mod util;
mod account_2;

const MAX_BYTECODE_SIZE_1: usize = 180000;

#[tokio::main]
async fn main() {

    //util_test().await;
    account_2::test().await;
}

pub async fn util_test(){
    let str = "execute";
    // 0x65786563757465
    let felt = util::get_felt_from_str(str).await;
    println!("felt: {:?}", felt.to_hex_string());
    let str = util::get_str_from_felt(felt).await;
    println!("str: {:?}", str);

    println!("dojo_starter");
    println!("{}", util::compute_bytearray_hash("dojo_starter").to_hex_string())


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

    word_contract::permissions().await;
    //word_contract::init_contract().await;
}

pub async  fn contract_call_test() {
    //println!("contract_call_test");

    my_contract::call_spawn().await;
}

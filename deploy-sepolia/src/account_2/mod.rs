pub mod account_2_register;
pub mod account_2_permission;
mod account_2_contract_call;
mod account_1_contract_call;

pub(crate) async fn test() {
    //account_2::account_2_register::decare().await;
    //account_2::account_2_register::deploy().await;
    //account_2_register::register_namespace().await;
    //account_2_register::register_model_decare_move().await;
    //account_2_register::register_model_move().await;
    //account_2_register::register_model_decare_position().await;
    //account_2_register::register_model_position().await;
    //account_2_register::register_model_decare_DirectionsAvailable().await;
    //account_2_register::register_model_DirectionsAvailable().await;
    //account_2_register::register_event_decare_move().await;
    //account_2_register::register_event_move().await;
    //account_2_register::register_contract_decare_actions().await;
    //account_2_register::register_contract_actions().await;
    //account_2_permission::permissions().await;
    //account_2_contract_call::spawn_1_call().await;
    account_2_contract_call::move_call().await;
    //account_1_contract_call::spawn_call().await;
    //account_1_contract_call::move_call().await;
}
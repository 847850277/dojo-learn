mod contract_call;

pub(crate) async fn test() {
    //contract_call::read_call_get_session_id().await;
    contract_call::write_call_create_lobby().await;
}
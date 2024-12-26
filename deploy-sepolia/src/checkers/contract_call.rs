use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Call, Felt, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::macros::{felt, selector};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

pub(crate) async fn read_call_get_session_id() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        //Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
        Url::parse("http://localhost:5050/").unwrap(),
    ));

    let contract_address =
        //felt!("0x03001dc519d85a62b71355e3a8dfa30c0fe526a3687fbfa8499ea604ba7d9aec");
        felt!("0x3001dc519d85a62b71355e3a8dfa30c0fe526a3687fbfa8499ea604ba7d9aec");

    let call_result =
        provider
            .call(
                FunctionCall {
                    contract_address: contract_address,
                    entry_point_selector: selector!("get_session_id"),
                    calldata: vec![
                       // Felt::from_hex("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap()
                    ],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .expect("failed to call contract");

    dbg!(call_result);
}

pub(crate) async fn write_call_create_lobby() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("http://localhost:5050/").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0xc5b2fcab997346f3ea1c00b002ecf6f382c5f9c9659a3894eb783c5320f912").unwrap(),
    ));
    let address = Felt::from_hex("0x127fd5f1fe78a71f8bcd1fec63e3fe2f0486b6ecd5c86a0466c3a21fa5cfcec").unwrap();
    let contract_address =
        Felt::from_hex("0x3001dc519d85a62b71355e3a8dfa30c0fe526a3687fbfa8499ea604ba7d9aec").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute_v1(vec![Call {
            to: contract_address,
            selector: get_selector_from_name("create_lobby").unwrap(),
            calldata: vec![
                //felt!("1")
                // Felt::from_dec_str("1000000000000000000000").unwrap(),
                // Felt::ZERO,
            ],
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::core::utils::get_selector_from_name;
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

pub(crate) async fn spawn_1_call() {
    todo!()
}

pub(crate) async fn move_call() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let contract_address =
        Felt::from_hex("0x023edf322be9eb54e407080d6eda97a2a5d3262a823bc9ae94fbac22a3464d9a").unwrap();

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
            selector: get_selector_from_name("move").unwrap(),
            calldata: vec![
                felt!("2")
                // Felt::from_dec_str("1000000000000000000000").unwrap(),
                // Felt::ZERO,
            ],
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
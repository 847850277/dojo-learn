use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::macros::felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

// 账户2调用世界合约的权限
pub(crate) async fn permissions() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x028a46eddc7615d00e21d31dc959d2721c3cc5b267e381b7fd4c7931f3e61dfe").unwrap(),
    ));
    let address = Felt::from_hex("0x04097f4882C50bDdBaFe1A79337bDaBDf001456430aDede37F36E47E22d135De").unwrap();
    let word_contract_address =
        Felt::from_hex("0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52").unwrap();

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

    //
    let mut calldata = vec![];

    // selector 地址为 namespace
    //let selector = starknet::macros::selector!("actions");//get_selector_from_name("actions").unwrap();
    let selector = felt!("0x6234d36dccb55b4b11d8862762d17515213173cbdf5427818157e8c5f45f6a2");
    // contract address
    let contract_address = felt!("0x023edf322be9eb54e407080d6eda97a2a5d3262a823bc9ae94fbac22a3464d9a");
    //calldata.insert(contract_address);
    calldata.extend(vec![selector]);
    //calldata.push(contract_address);
    calldata.extend(vec![contract_address]);
    //calldata.push(selector);

    // print selector && contract address
    println!("selector: {}", selector.to_hex_string());
    println!("contract_address: {}", contract_address.to_hex_string());

    let result = account
        .execute_v1(vec![Call {
            to: word_contract_address,
            selector: starknet::macros::selector!("grant_writer"),//get_selector_from_name("grant_writer").unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
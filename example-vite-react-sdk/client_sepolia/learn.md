# sepolia_demo

## provider 为sepolia

## 账户为sepolia的已知账户

## 列表调整为固定的某个账户
```bash

{account_array.map((account, index) => (
                                <option value={account.address} key={index}>
                                    {account.address}
                                </option>
                            ))}
```



### rpc的fee方法报错

- 本地网络正确的请求
```json
{"id":5,"jsonrpc":"2.0","method":"starknet_estimateFee","params":{"request":[{"type":"INVOKE","sender_address":"0x77b53ecfbb3ba8e95bcfabb7e59ba15983fddda36b4c0c4c938f965b1668e99","calldata":["0x1","0x58b83bea84766c5725c12e239c1ec9e1fde679ddf709b772fe7a8fdfd3cda27","0x217c73ea9ef26581623f20edd45571c1d024612b70d0af3e0842c5b0dc253cd","0x0"],"version":"0x100000000000000000000000000000001","signature":[],"nonce":"0x1","max_fee":"0x0"}],"block_id":"pending","simulation_flags":["SKIP_VALIDATE"]}}

```

- 响应
```json

{
    "jsonrpc": "2.0",
    "result": [
        {
            "gas_consumed": "0x16ae",
            "gas_price": "0x4a817c800",
            "data_gas_consumed": "0x0",
            "data_gas_price": "0x0",
            "overall_fee": "0x699c4b59f000",
            "unit": "WEI"
        }
    ],
    "id": 5
}

```

### 执行合约函数

```json 请求
{"id":6,"jsonrpc":"2.0","method":"starknet_addInvokeTransaction","params":{"invoke_transaction":{"sender_address":"0x77b53ecfbb3ba8e95bcfabb7e59ba15983fddda36b4c0c4c938f965b1668e99","calldata":["0x1","0x58b83bea84766c5725c12e239c1ec9e1fde679ddf709b772fe7a8fdfd3cda27","0x217c73ea9ef26581623f20edd45571c1d024612b70d0af3e0842c5b0dc253cd","0x0"],"type":"INVOKE","max_fee":"0x9e6a7106e800","version":"0x1","signature":["0x4f7bf5f4759823733121b8c15cce359953a409a03094dd0b4075cfbc1425ce7","0x19a9f210d995d510349888b49899a854b547ea0f9ae46154fa0ad5fada31e3c"],"nonce":"0x1"}}}

```

```json 响应
{
    "jsonrpc": "2.0",
    "result": {
        "transaction_hash": "0x3f85da7b3ce6ee38b52151086202bbfb9aaf8a65e14edf7a44800f2d0c8c90b"
    },
    "id": 6
}

```
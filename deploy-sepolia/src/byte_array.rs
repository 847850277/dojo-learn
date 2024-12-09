use starknet::core::types::Felt;
use starknet::core::utils::get_selector_from_name;
use starknet::macros::felt;


// byte_array的数据格式
// 0x
// 值
// 值的长度
pub async fn byte_array() -> Vec<Felt> {
    vec![
        felt!("0x00"),
        // 字符串a，16进制编码为0x61,2进制为97
        felt!("0x61"),
        // 字符串a的长度为1
        felt!("0x01"),
    ]
}

// 字符串转16进制编码
pub async  fn string_to_hex(input: &str) -> String {
    input.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
}


pub async fn byte_array_str(input: &str) -> Vec<Felt> {
    let hex: String = string_to_hex(input).await;
    let hex = "0x".to_string() + &hex;
    let len = format!("0x{:02x}", input.len());
    //let len = "0x".to_string() + &len;
    vec![
        felt!("0x00"),
        Felt::from_hex(&hex).unwrap(),
        Felt::from_hex(&len).unwrap(),
    ]
}
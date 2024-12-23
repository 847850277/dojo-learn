use starknet::core::types::Felt;
use starknet::core::utils::{cairo_short_string_to_felt, parse_cairo_short_string};

pub async fn get_str_from_felt(felt: Felt) -> String {
    // to Str
    //0x7a1c71029f2d0b38e3ac89b09931d08b6e48417e079c289ff19a8698d0cba33
    let result = parse_cairo_short_string(&felt);
    match result {
        Ok(s) => {
            return s;
        }
        Err(e) => {
            return "".to_string();
        }
    }
}

pub async fn get_felt_from_str(str: &str) -> Felt {
    // to felt
    cairo_short_string_to_felt(str).unwrap()

}
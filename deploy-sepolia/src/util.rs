use starknet::core::types::Felt;

pub fn get_str_from_felt(felt: Felt) -> String {
    // to Str
    //0x7a1c71029f2d0b38e3ac89b09931d08b6e48417e079c289ff19a8698d0cba33
    felt.to_hex_string()
}
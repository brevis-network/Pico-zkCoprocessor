use crate::data_types::address::Address;
use crypto_bigint::U256;

/// convert u256 to u32 vec
pub fn u256_to_u32_vec(data: U256) -> Vec<u32> {
    // index slice one by one more cheapter than using bit operation or chunk options
    let bytes: [u8; 32] = data.to_be_bytes();
    let value0 = bytes[0] as u32 * 65536 + bytes[1] as u32 * 256 + bytes[2] as u32;
    let value1 = bytes[3] as u32 * 65536 + bytes[4] as u32 * 256 + bytes[5] as u32;
    let value2 = bytes[6] as u32 * 65536 + bytes[7] as u32 * 256 + bytes[8] as u32;
    let value3 = bytes[9] as u32 * 65536 + bytes[10] as u32 * 256 + bytes[11] as u32;
    let value4 = bytes[12] as u32 * 65536 + bytes[13] as u32 * 256 + bytes[14] as u32;
    let value5 = bytes[15] as u32 * 65536 + bytes[16] as u32 * 256 + bytes[17] as u32;
    let value6 = bytes[18] as u32 * 65536 + bytes[19] as u32 * 256 + bytes[20] as u32;
    let value7 = bytes[21] as u32 * 65536 + bytes[22] as u32 * 256 + bytes[23] as u32;
    let value8 = bytes[24] as u32 * 65536 + bytes[25] as u32 * 256 + bytes[26] as u32;
    let value9 = bytes[27] as u32 * 65536 + bytes[28] as u32 * 256 + bytes[29] as u32;
    let value10 = bytes[30] as u32 * 256 + bytes[31] as u32;

    vec![
        value0, value1, value2, value3, value4, value5, value6, value7, value8, value9, value10,
    ]
}

/// convert address to u32 vec
pub fn address_to_u32_vec(data: Address) -> Vec<u32> {
    let value0 = data[0] as u32 * 65536 + data[1] as u32 * 256 + data[2] as u32;
    let value1 = data[3] as u32 * 65536 + data[4] as u32 * 256 + data[5] as u32;
    let value2 = data[6] as u32 * 65536 + data[7] as u32 * 256 + data[8] as u32;
    let value3 = data[9] as u32 * 65536 + data[10] as u32 * 256 + data[11] as u32;
    let value4 = data[12] as u32 * 65536 + data[13] as u32 * 256 + data[14] as u32;
    let value5 = data[15] as u32 * 65536 + data[16] as u32 * 256 + data[17] as u32;
    let value6 = data[18] as u32 * 256 + data[19] as u32;

    vec![value0, value1, value2, value3, value4, value5, value6]
}

/// convert topic to u32 vec
pub fn topic_to_u32_vec(data: &[u8]) -> Vec<u32> {
    if data.len() != 6 {
        panic!("topic length must be 6");
    }
    let value0 = data[0] as u32 * 65536 + data[1] as u32 * 256 + data[2] as u32;
    let value1 = data[3] as u32 * 65536 + data[4] as u32 * 256 + data[5] as u32;
    vec![value0, value1]
}

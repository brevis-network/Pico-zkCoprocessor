use crate::Hex;

pub type Bytes32 = [u8; 32];

impl Hex for Bytes32 {
    fn from_hex(hex_str: &str) -> Result<Self, &'static str> {
        let mut hex_str = hex_str;
        if hex_str.starts_with("0x") {
            hex_str = &hex_str[2..];
        }
        if hex_str.len() != 64 {
            return Err("Invalid length of hex string");
        }

        let mut res: [u8; 32] = [0; 32];
        for (i, chunk) in hex_str.as_bytes().chunks(2).enumerate() {
            let byte = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16)
                .map_err(|_| "Invalid hex character")?;
            res[i] = byte;
        }

        Ok(res)
    }

    fn to_hex(&self) -> String {
        let mut hex_string = String::with_capacity(66);
        hex_string.push_str("0x");
        for &byte in self.iter() {
            hex_string.push_str(&format!("{:02x}", byte));
        }
        hex_string
    }
}
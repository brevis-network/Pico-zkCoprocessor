pub mod data_types;
pub mod input_types;
pub mod inputs_merkle;
pub mod poseidon2_hash;
pub mod sdk;

use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32[8] input_commitment;
    }
}

pub trait Hex: Sized {
    // Convert from a big-endian hex string
    fn from_hex(hex_str: &str) -> Result<Self, &'static str>;

    // Convert to a big-endian hex string
    fn to_hex(&self) -> String;
}

#[cfg(target_os = "zkvm")]
extern "C" {
    pub fn syscall_poseidon2_permute(x: *const [u32; 16], y: *mut [u32; 16]);
}

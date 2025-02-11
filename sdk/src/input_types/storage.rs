use crypto_bigint::U256;
use serde::{Deserialize, Serialize};

use crate::{
    data_types::{
        address::Address,
        byte32::Bytes32,
        hash_out::{MerkleInput, HASH_OUT},
    },
    poseidon2_hash::Poseidon2,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSlotData {
    pub block_num: u32,
    pub block_base_fee: U256,
    pub block_time: u32,
    pub address: Address,
    pub slot: Bytes32,
    pub value: U256,
}

impl StorageSlotData {
    pub fn add_storage_slot(block_num: u32, block_base_fee: U256, block_time: u32, address: Address, slot: Bytes32, value: U256) -> Self {
        StorageSlotData {
            block_num,
            block_base_fee,
            block_time,
            address,
            slot,
            value,
        }
    }

    pub fn commit_inputs(&self) -> MerkleInput {
        let mut inputs: Vec<u32> = vec![];
        inputs.push(self.block_num);
        let base_fee_le = self.block_base_fee.to_be_bytes();
        let base_fee_elements = base_fee_le
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(base_fee_elements);
        inputs.push(self.block_time);
        let address_elements = self
            .address
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(address_elements);

        let slots_elements = self
            .slot
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(slots_elements);

        let value_elements = self
            .value
            .to_be_bytes()
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(value_elements);

        Poseidon2::<HASH_OUT>::hash(inputs.as_slice())
    }
}

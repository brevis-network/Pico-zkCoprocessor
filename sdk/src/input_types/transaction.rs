use crypto_bigint::U256;
use serde::{Deserialize, Serialize};

use crate::{
    data_types::{
        byte32::Bytes32,
        hash_out::{MerkleInput, HASH_OUT},
    },
    poseidon2_hash::Poseidon2,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub hash: Bytes32,
    pub block_num: u32,
    pub block_base_fee: U256,
    pub block_time: u32,
    pub mpt_key_path: u32,
    pub leaf_hash: Bytes32,
}

impl TransactionData {
    pub fn add_transaction(
        hash: Bytes32,
        block_num: u32,
        block_base_fee: U256,
        block_time: u32,
        mpt_key_path: u32,
        leaf_hash: Bytes32,
    ) -> Self {
        TransactionData {
            hash,
            block_num,
            block_base_fee,
            block_time,
            mpt_key_path,
            leaf_hash,
        }
    }

    pub fn commit_inputs(&self) -> MerkleInput {
        let mut inputs: Vec<u32> = vec![];

        let hash_elements = self
            .hash
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(hash_elements);
        inputs.push(self.block_num);
        let base_fee_le = self.block_base_fee.to_be_bytes();
        let base_fee_elements = base_fee_le
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(base_fee_elements);
        inputs.push(self.block_time);
        inputs.push(self.mpt_key_path);

        let leaf_hash_elements = self
            .leaf_hash
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        inputs.extend(leaf_hash_elements);
        Poseidon2::<HASH_OUT>::hash(inputs.as_slice())
    }
}

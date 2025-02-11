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

use super::utils::{address_to_u32_vec, topic_to_u32_vec, u256_to_u32_vec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptData {
    pub transaction_hash: Bytes32,
    pub block_num: u32,
    pub block_base_fee: U256,
    pub block_time: u32,
    pub mpt_key_path: u32,
    pub fields: Vec<LogFieldData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFieldData {
    pub contract: Address,
    pub topic: Bytes32,
    pub log_pos: u32,
    pub is_topic: bool,
    pub field_index: u32,
    pub value: U256,
}

impl ReceiptData {
    pub fn add_receipt(
        transaction_hash: Bytes32,
        block_num: u32,
        block_base_fee: U256,
        block_time: u32,
        mpt_key_path: u32,
        fields: Vec<LogFieldData>,
    ) -> Self {
        ReceiptData {
            transaction_hash: transaction_hash,
            block_num,
            block_base_fee,
            block_time,
            mpt_key_path,
            fields,
        }
    }

    pub fn commit_inputs(&self) -> MerkleInput {
        let mut inputs: Vec<u32> = vec![];
        inputs.push(self.block_num);
        let base_fee_elements = u256_to_u32_vec(self.block_base_fee);
        inputs.extend(base_fee_elements);
        inputs.push(self.block_time);
        inputs.push(self.mpt_key_path);

        for field in self.fields.as_slice() {

            let contract_elements = address_to_u32_vec(field.contract);
            inputs.extend(contract_elements);

            let topic_elements = topic_to_u32_vec(&field.topic[..6]);
            inputs.extend(topic_elements);

            inputs.push(field.log_pos);
            let is_topic = if field.is_topic { 1 } else { 0 };
            inputs.push(is_topic);
            inputs.push(field.field_index);
    
            let value_elements = u256_to_u32_vec(field.value);
            inputs.extend(value_elements);
        }

        Poseidon2::<HASH_OUT>::hash(inputs.as_slice())
    }
}

#[cfg(test)]
mod test {
    use crypto_bigint::U256;

    use crate::{
        data_types::{address::Address, byte32::Bytes32},
        input_types::receipt::{LogFieldData, ReceiptData},
        Hex,
    };

    #[test]
    fn test_add_receipt() {
        let transaction_hash = "0x32a3411c96bae73b2779a3f27a5d8864a1ffe4a471a0e50299e7aaf1681b177e";
        let transaction_hash = Bytes32::from_hex(&transaction_hash).unwrap();

        println!("transactin hash: {:?}", transaction_hash.to_hex());

        let contract_hex = "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
        let contract = Address::from_hex(contract_hex).unwrap();

        let topic_hex = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
        let topic: Bytes32 = Bytes32::from_hex(&topic_hex).unwrap();

        let base_fee_str = "0000000000000000000000000000000000000000000000051a69508568586800";
        let base_fee = U256::from_be_hex(base_fee_str);

        let block_num = 21135753;
        let block_time = 1732158058;

        let fields = vec![LogFieldData {
            contract,
            topic,
            log_pos: 1,
            field_index: 1,
            is_topic: true,
            value: U256::from_be_hex(
                "0000000000000000000000000000000000000000000000000000000000000064",
            ),
        }];

        let mpt_key_path = 3482948;

        let receipt = ReceiptData::add_receipt(
            transaction_hash,
            block_num,
            base_fee,
            block_time,
            mpt_key_path,
            fields,
        );

        println!("receipt: {:?}", receipt);
    }
}

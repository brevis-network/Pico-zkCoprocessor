use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Error, Ok};
use crypto_bigint::{Zero, U256};
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use serde::{Deserialize, Serialize};

const RECEIPT_DUMMYS: [(u64, &str); 1] = [
    (
        1,
        "0x49d4b43790b25474",
    )
];

const STORAGE_DUMMYS: [(u64, &str); 1] = [
    (
        1,
        "0x4e93157937ed2e80",
    )
];

const TX_DUMMYS: [(u64, &str); 1] = [
    (
        1,
        "0x46af2cafe7df8b99",
    )
];

use crate::{
    data_types::hash_out::MerkleInput,
    input_types::{receipt::ReceiptData, storage::StorageSlotData, transaction::TransactionData},
    inputs_merkle::build_merkle_tree,
};

#[derive(Default)]
pub struct Builder {
    pub chain_id: u64,
    pub receipts: Option<Vec<ReceiptData>>,
    pub storage_slots: Option<Vec<StorageSlotData>>,
    pub transactions: Option<Vec<TransactionData>>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn with_receipts(mut self, receipts: Vec<ReceiptData>) -> Self {
        self.receipts = Some(receipts);
        self
    }

    pub fn with_storage_slots(mut self, storage_slots: Vec<StorageSlotData>) -> Self {
        self.storage_slots = Some(storage_slots);
        self
    }

    pub fn with_transactions(mut self, transactions: Vec<TransactionData>) -> Self {
        self.transactions = Some(transactions);
        self
    }

    pub fn init(self, max_receipt_size: u32, max_storage_size: u32, max_tx_size: u32) -> SDK {
        if let Some(receipts) = self.receipts.clone() {
            if receipts.len() > max_receipt_size as usize {
                panic!(
                    "receipts length exceeds the max receipt size: {:?}",
                    max_receipt_size
                );
            }
        }

        if let Some(storage_slots) = self.storage_slots.clone() {
            if storage_slots.len() > max_storage_size as usize {
                panic!(
                    "receipts length exceeds the max storage size: {:?}",
                    max_storage_size
                );
            }
        }

        if let Some(transactions) = self.transactions.clone() {
            if transactions.len() > max_tx_size as usize {
                panic!(
                    "receipts length exceeds the max transaction size: {:?}",
                    max_tx_size
                );
            }
        }

        SDK::new(
            self.chain_id,
            max_receipt_size,
            max_storage_size,
            max_tx_size,
            self.receipts,
            self.storage_slots,
            self.transactions,
        )
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SDK {
    pub chain_id: u64,
    pub max_receipt_size: u32,
    pub max_storage_size: u32,
    pub max_tx_size: u32,
    pub receipts: Option<Vec<ReceiptData>>,
    pub storage_slots: Option<Vec<StorageSlotData>>,
    pub transactions: Option<Vec<TransactionData>>,
    pub dummy_commitments: DummyCommitment,
    pub input_commitments: MerkleInput,
    /// Indicate whether input_commitment has been committed
    commited: bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DummyCommitment {
    pub receipt_dummy_commitment: [u32; 8],
    pub storage_dummy_commitment: [u32; 8],
    pub tx_dummy_commitment: [u32; 8],
}

impl DummyCommitment {
    fn from_str(hex_str: &str) -> [u32; 8] {
        let bytes = hex::decode(&hex_str[2..]).unwrap();
        let dummy_fields = bytes
            .chunks_exact(4)
            .map(|chunk| (chunk[0] as u32) * 256 + chunk[1] as u32)
            .collect::<Vec<_>>();
        dummy_fields.try_into().unwrap()
    }

    pub fn get_dummy_commitment(chain_id: u64) -> DummyCommitment {
        let zero_dummy_hex = "0x0000000000000000000000000000000000000000000000000000000000000000";
        let receipt_dummy_map: HashMap<u64, &str> = RECEIPT_DUMMYS.iter().cloned().collect();
        let receipt_dummy = receipt_dummy_map.get(&chain_id).unwrap_or(&zero_dummy_hex);
        let receipt_dummy_commitment = Self::from_str(receipt_dummy);

        let storage_dummy_map: HashMap<u64, &str> = STORAGE_DUMMYS.iter().cloned().collect();
        let storage_dummy = storage_dummy_map.get(&chain_id).unwrap_or(&zero_dummy_hex);
        let storage_dummy_commitment = Self::from_str(&storage_dummy);

        let tx_dummy_map: HashMap<u64, &str> = TX_DUMMYS.iter().cloned().collect();
        let tx_dummy = tx_dummy_map.get(&chain_id).unwrap_or(&zero_dummy_hex);
        let tx_dummy_commitment = Self::from_str(&tx_dummy);

        DummyCommitment {
            receipt_dummy_commitment,
            storage_dummy_commitment,
            tx_dummy_commitment,
        }
    }
}

impl SDK {
    fn new(
        chain_id: u64,
        max_receipt_size: u32,
        max_storage_size: u32,
        max_tx_size: u32,
        receipts: Option<Vec<ReceiptData>>,
        storage_slots: Option<Vec<StorageSlotData>>,
        transactions: Option<Vec<TransactionData>>,
    ) -> Self {
        let dummy_commitments = DummyCommitment::get_dummy_commitment(chain_id);
        let mut sdk = SDK {
            chain_id,
            max_receipt_size,
            max_storage_size,
            max_tx_size,
            receipts,
            storage_slots,
            transactions,
            dummy_commitments,
            input_commitments: MerkleInput::default(),
            commited: false,
        };
        sdk.commit_inputs();
        sdk
    }

    /// Commits all the inputs and get the root hash
    fn commit_inputs(&mut self) {
        let leaves_size = self.max_receipt_size + self.max_storage_size + self.max_tx_size;
        let mut leaves = Vec::with_capacity(leaves_size as usize);

        // commit receipts and pad with dummy receipts
        if let Some(receipts) = self.receipts.clone() {
            receipts.iter().for_each(|d| {
                leaves.push(d.commit_inputs());
            });
            println!("receipt commitment: {:?}", leaves[0])
        }
        for _ in leaves.len()..self.max_receipt_size as usize {
            leaves.push(self.dummy_commitments.receipt_dummy_commitment);
        }

        // commit storage and pad with dummy storage
        if let Some(storages) = self.storage_slots.clone() {
            storages.iter().for_each(|d| {
                leaves.push(d.commit_inputs());
            });
        }
        for _ in leaves.len()..self.max_receipt_size as usize + self.max_storage_size as usize {
            leaves.push(self.dummy_commitments.storage_dummy_commitment);
        }

        // commit transactions and pad with dummy transactions.
        if let Some(transactions) = self.transactions.clone() {
            transactions.iter().for_each(|d| {
                leaves.push(d.commit_inputs());
            });
        }
        for _ in leaves.len()
            ..self.max_receipt_size as usize
                + self.max_storage_size as usize
                + self.max_tx_size as usize
        {
            leaves.push(self.dummy_commitments.tx_dummy_commitment);
        }
        // println!("leaves length: {:?}", leaves.len());
        // build merkle root
        let root = build_merkle_tree(&leaves);
        self.input_commitments = root.hash;
    }

    /// Save the input data inputs into file, prepare for proving the input data by brevis zk
    /// this function can't be used in the riscv program
    /// once the inputs be saved, use this file to request proving the inputs REQUEST_DATA_FILE=file-path brevis-request
    pub fn save_inputs(&self, dir: PathBuf) -> anyhow::Result<PathBuf, Error> {
        let file_path = dir.join("request_prove_inputs.json");
        let file = fs::File::create(file_path.clone()).expect("create request file failed");
        serde_json::to_writer(file, self)?;
        Ok(file_path)
    }

    pub fn sum_of_u256(&self, values: Vec<U256>) -> U256 {
        let mut sum: U256 = U256::zero();
        values.iter().for_each(|v| {    
            sum = sum.add_mod(v, &U256::from_u32(BabyBear::ORDER_U32));
        });
        sum
    }

    pub fn set_commited_status(&mut self, is_commited: bool) {
        self.commited = is_commited
    }

    pub fn is_commited(&self) -> bool{
        self.commited
    }

}

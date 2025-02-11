use receipt::ReceiptData;
use serde::{Deserialize, Serialize};
use storage::StorageSlotData;
use transaction::TransactionData;


pub mod receipt;
pub mod storage;
pub mod transaction;
pub mod utils;


#[derive(Clone, Serialize, Deserialize)]
pub struct InputRequestData {
    receipt_size: usize,
    storage_size: usize,
    tx_size: usize,
    receipts: Option<Vec<ReceiptData>>,
    storage_slots: Option<Vec<StorageSlotData>>,
    transactions: Option<Vec<TransactionData>>,
}
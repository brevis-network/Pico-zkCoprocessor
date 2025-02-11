use coprocessor_sdk::{
    input_types::{receipt::ReceiptData, storage::StorageSlotData, transaction::TransactionData},
    Hex,
};

use crate::gateway::{
    LogExtractInfo, Query, ReceiptInfo, SendBatchQueriesRequest, StorageQueryInfo, TransactionInfo, VmAppCircuitInfo,
};

pub fn prepare_request(
    chain_id: u64,
    receipts: Option<Vec<ReceiptData>>,
    storage_slots: Option<Vec<StorageSlotData>>,
    txs: Option<Vec<TransactionData>>,
    limits: Vec<u32>, // max receipts, max storage slots, max txs
) -> SendBatchQueriesRequest {
    let mut receipt_infos = vec![];
    if let Some(receipts) = receipts {
        receipt_infos = receipts
            .iter()
            .map(|receipt| {
                let log_extract_infos = receipt
                    .fields
                    .iter()
                    .map(|log| LogExtractInfo {
                        contract_address: log.contract.to_hex(),
                        log_pos: log.log_pos as u64,
                        log_topic0: log.topic.to_hex(),
                        value_from_topic: log.is_topic,
                        value_index: log.field_index as u64,
                        value: log.value.to_string(),
                        topics_length: 0, //deprecated
                    })
                    .collect::<Vec<_>>();

                ReceiptInfo {
                    transaction_hash: receipt.transaction_hash.to_hex(),
                    log_extract_infos: log_extract_infos,
                    blk_num: receipt.block_num as u64,
                    receipt_index: receipt.mpt_key_path as u64,
                }
            })
            .collect::<Vec<_>>();
    }

    let mut storage_query_infos = vec![];
    if let Some(storage_slot) = storage_slots {
        storage_query_infos = storage_slot
            .iter()
            .map(|slot| StorageQueryInfo {
                account: slot.address.to_hex(),
                storage_keys: vec![slot.slot.to_hex()],
                blk_num: slot.block_num as u64,
            })
            .collect::<Vec<_>>()
    }

    let mut transaction_infos = vec![];
    if let Some(txs) = txs {
        transaction_infos = txs
            .iter()
            .map(|tx| TransactionInfo {
                transaction_hash: tx.hash.to_hex(),
            })
            .collect::<Vec<_>>()
    };

    let mut vm_app_info = VmAppCircuitInfo::default();
    vm_app_info.max_receipts = limits[0];
    vm_app_info.max_storage = limits[1];
    vm_app_info.max_tx = limits[2];
    let mut total: u32 = limits.iter().sum();
    if total %2 != 0 {
        total += 1;
    }
    vm_app_info.max_num_data_points = total;
    
    let query = Query {
        receipt_infos,
        storage_query_infos: storage_query_infos,
        transaction_infos: transaction_infos,
        app_circuit_info: None,
        vm_app_circuit_info: Some(vm_app_info),
        use_plonky2: true,
    };

    SendBatchQueriesRequest {
        chain_id,
        queries: vec![query],
        target_chain_id: chain_id,
        option: 0, // zk mode
        api_key: "TESTVM".to_string(),
        use_vm: true,
    }
}

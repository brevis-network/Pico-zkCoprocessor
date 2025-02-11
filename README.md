# coprocessor-vm-sdk

PICO-VM based coprocessor SDK.

## Quick start ##

1. Import pico-sdk with coprocessor

```
pico-sdk = { git = "https://github.com/brevis-network/brevis-vm.git", features = ["coprocessor"]}
```
Enable feature "coprocessor" to apply coprocessor plugin in pico-sdk

- Add PICO Coprocessor SDK dependency
```
coprocessor-sdk = { git = "https://github.com/brevis-network/coprocessor-vm-sdk.git"}
```

2. SDK Initialize
   
```rust
let sdk = &Builder::new()
         .with_receipts(receipts) // optional
         .with_storage_slots(storage_slots) // optional
         .with_transactions(transactions) // optional
         .init(MAX_RECEIPT_SIZE, MAX_STORAGE_SIZE, MAX_TX_SIZE);
```

`MAX_XX_SIZE`: Specifies the size of each input data type maximum size. It must be a multiple of 32



3. Example of calculating the sum(values) of all transaction receipts.
```rust
if let Some(receipts) = sdk.receipts.clone() {
        let values: Vec<U256> = receipts.iter().map(|it| it.fields[0].value).collect();
        let result = sdk.sum_of_u256(values);
        println!("public value: {:?}", &result.clone().to_string());
} 
```

4. Commit to public inputs 
```rust
pico_sdk::io::commit_coprocessor_bytes(&mut sdk, &mut result.to_be_bytes());
```

If you commit a struct data
```rust
pico_sdk::io::commit_coprocessor_value(&mut sdk, &value);
```


## Build and prove example program ##

Build program
```shell
cd example
RUST_LOG=info cargo pico build
```

Prove
```shell
RUST_LOG=info cargo pico prove --fast
```

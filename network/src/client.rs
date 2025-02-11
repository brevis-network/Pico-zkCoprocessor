pub mod request;
use std::env;
use std::fs::File;

use coprocessor_sdk::sdk::SDK;
use gateway::gateway_client::GatewayClient;
use request::prepare_request;
use std::io::BufReader;

pub mod gateway {
    tonic::include_proto!("brevis");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_json = env::var("REQUEST_DATA_FILE").expect("REQUST_DATA_FILE not set");
    // Open the file
    let file = File::open(input_json)?;
    let reader = BufReader::new(file);
    let input_data: SDK = serde_json::from_reader(reader).unwrap();

    let limits = vec![
        input_data.max_receipt_size,
        input_data.max_storage_size,
        input_data.max_tx_size,
    ];
    let request = prepare_request(
        input_data.chain_id,
        input_data.receipts,
        input_data.storage_slots,
        input_data.transactions,
        limits
    );
    println!("request: {:?}", request);
    let mut client: GatewayClient<tonic::transport::Channel> =
        GatewayClient::connect("https://appsdkv3.brevis.network:443").await?;

    let request = tonic::Request::new(request);
    let response = client.send_batch_queries_async(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

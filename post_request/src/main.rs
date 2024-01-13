// post request to solana api devnet
extern crate reqwest;

use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let json_body =
        json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx", // wallet address
            {
                "encoding": "base58",
            }
        ]
    });

    let res = client
        .post("https://api.devnet.solana.com")
        .json(&json_body)
        .send().await
        .expect("failed to get response")
        .text().await
        .expect("failed to get payload");

    println!("{:?}", res);

    Ok(())
}

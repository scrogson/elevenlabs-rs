#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let history_item_ids = &["your history item ids"];
    let response = client
        .download_history_items_v1_history_download_post(history_item_ids)
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let history_item_id = "your history item id";
    let response = client
        .delete_history_item_v1_history_history_item_id_delete(history_item_id)
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

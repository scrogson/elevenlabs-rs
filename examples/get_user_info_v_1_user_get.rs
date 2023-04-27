#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let response = client
        .get_user_info_v1_user_get()
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

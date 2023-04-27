#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let response = client
        .add_voice_v1_voices_add_post()
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

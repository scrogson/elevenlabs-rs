#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let voice_id = "your voice id";
    let response = client
        .delete_voice_v1_voices_voice_id_delete(voice_id)
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

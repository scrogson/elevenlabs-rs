#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let response = client
        .get_default_voice_settings_v1_voices_settings_default_get()
        .await
        .unwrap();
    println!("{:#?}", response);
}

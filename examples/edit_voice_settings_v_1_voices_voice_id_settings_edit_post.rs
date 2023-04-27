#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let similarity_boost = 1.0;
    let stability = 1.0;
    let voice_id = "your voice id";
    let response = client
        .edit_voice_settings_v1_voices_voice_id_settings_edit_post(
            similarity_boost,
            stability,
            voice_id,
        )
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

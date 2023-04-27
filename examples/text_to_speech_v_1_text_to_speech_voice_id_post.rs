#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let text = "your text";
    let voice_id = "your voice id";
    let response = client
        .text_to_speech_v1_text_to_speech_voice_id_post(text, voice_id)
        .voice_settings(VoiceSettingsResponseModel {
            similarity_boost: 1.0,
            stability: 1.0,
        })
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

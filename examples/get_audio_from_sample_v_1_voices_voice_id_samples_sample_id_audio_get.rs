#![allow(unused_imports)]
use elevenlabs::model::*;
use elevenlabs::ElevenLabsClient;
#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let sample_id = "your sample id";
    let voice_id = "your voice id";
    let response = client
        .get_audio_from_sample_v1_voices_voice_id_samples_sample_id_audio_get(sample_id, voice_id)
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}

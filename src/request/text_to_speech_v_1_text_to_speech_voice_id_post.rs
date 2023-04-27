use crate::model::*;
use crate::ElevenLabsClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TextToSpeechV1TextToSpeechVoiceIdPostRequest<'a> {
    pub(crate) http_client: &'a ElevenLabsClient,
    pub text: String,
    pub voice_id: String,
    pub voice_settings: Option<VoiceSettingsResponseModel>,
    pub xi_api_key: Option<String>,
}
impl<'a> TextToSpeechV1TextToSpeechVoiceIdPostRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post(&format!(
            "/v1/text-to-speech/{voice_id}",
            voice_id = self.voice_id
        ));
        r = r.json(json!({ "text" : self.text }));
        if let Some(ref unwrapped) = self.voice_settings {
            r = r.json(json!({ "voice_settings": unwrapped }));
        }
        if let Some(ref unwrapped) = self.xi_api_key {
            r = r.header("xi-api-key", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn voice_settings(mut self, voice_settings: VoiceSettingsResponseModel) -> Self {
        self.voice_settings = Some(voice_settings);
        self
    }
    pub fn xi_api_key(mut self, xi_api_key: &str) -> Self {
        self.xi_api_key = Some(xi_api_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TextToSpeechV1TextToSpeechVoiceIdPostRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

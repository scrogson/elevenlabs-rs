use crate::model::*;
use crate::ElevenLabsClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetVoiceV1VoicesVoiceIdGetRequest<'a> {
    pub(crate) http_client: &'a ElevenLabsClient,
    pub voice_id: String,
    pub with_settings: Option<bool>,
    pub xi_api_key: Option<String>,
}
impl<'a> GetVoiceV1VoicesVoiceIdGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<VoiceResponseModel> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v1/voices/{voice_id}", voice_id = self.voice_id));
        if let Some(ref unwrapped) = self.with_settings {
            r = r.query("with_settings", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.xi_api_key {
            r = r.header("xi-api-key", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn with_settings(mut self, with_settings: bool) -> Self {
        self.with_settings = Some(with_settings);
        self
    }
    pub fn xi_api_key(mut self, xi_api_key: &str) -> Self {
        self.xi_api_key = Some(xi_api_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetVoiceV1VoicesVoiceIdGetRequest<'a> {
    type Output = httpclient::InMemoryResult<VoiceResponseModel>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

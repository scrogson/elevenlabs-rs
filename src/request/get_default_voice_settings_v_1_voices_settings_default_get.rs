use crate::model::*;
use crate::ElevenLabsClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetDefaultVoiceSettingsV1VoicesSettingsDefaultGetRequest<'a> {
    pub(crate) http_client: &'a ElevenLabsClient,
}
impl<'a> GetDefaultVoiceSettingsV1VoicesSettingsDefaultGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<VoiceSettingsResponseModel> {
        let mut r = self.http_client.client.get("/v1/voices/settings/default");
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture
    for GetDefaultVoiceSettingsV1VoicesSettingsDefaultGetRequest<'a>
{
    type Output = httpclient::InMemoryResult<VoiceSettingsResponseModel>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

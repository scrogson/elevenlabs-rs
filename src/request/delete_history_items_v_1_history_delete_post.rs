use crate::model::*;
use crate::ElevenLabsClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteHistoryItemsV1HistoryDeletePostRequest<'a> {
    pub(crate) http_client: &'a ElevenLabsClient,
    pub history_item_ids: Vec<String>,
    pub xi_api_key: Option<String>,
}
impl<'a> DeleteHistoryItemsV1HistoryDeletePostRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self.http_client.client.post("/v1/history/delete");
        r = r.json(json!({ "history_item_ids" : self.history_item_ids }));
        if let Some(ref unwrapped) = self.xi_api_key {
            r = r.header("xi-api-key", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn xi_api_key(mut self, xi_api_key: &str) -> Self {
        self.xi_api_key = Some(xi_api_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for DeleteHistoryItemsV1HistoryDeletePostRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

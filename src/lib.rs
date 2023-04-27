//! [`ElevenLabsClient`](struct.ElevenLabsClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct ElevenLabsClient {
    pub client: httpclient::Client,
    authentication: ElevenLabsAuthentication,
}
impl ElevenLabsClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url(
                std::env::var("ELEVEN_LABS_BASE_URL")
                    .expect("Missing environment variable ELEVEN_LABS_BASE_URL")
                    .as_str(),
            ),
            authentication: ElevenLabsAuthentication::from_env(),
        }
    }
}
impl ElevenLabsClient {
    pub fn new(url: &str, authentication: ElevenLabsAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self {
            client,
            authentication,
        }
    }
    pub fn with_authentication(mut self, authentication: ElevenLabsAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {}
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(mut self, middleware: M) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Get Generated Items

    Returns metadata about all your generated audio.*/
    pub fn get_generated_items_v1_history_get(
        &self,
    ) -> request::GetGeneratedItemsV1HistoryGetRequest {
        request::GetGeneratedItemsV1HistoryGetRequest {
            http_client: &self,
            xi_api_key: None,
        }
    }
    /**Get Audio From History Item

    Returns the audio of an history item.*/
    pub fn get_audio_from_history_item_v1_history_history_item_id_audio_get(
        &self,
        history_item_id: &str,
    ) -> request::GetAudioFromHistoryItemV1HistoryHistoryItemIdAudioGetRequest {
        request::GetAudioFromHistoryItemV1HistoryHistoryItemIdAudioGetRequest {
            http_client: &self,
            history_item_id: history_item_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Delete History Items

    Delete a number of history items by their IDs.*/
    pub fn delete_history_items_v1_history_delete_post(
        &self,
        history_item_ids: &[&str],
    ) -> request::DeleteHistoryItemsV1HistoryDeletePostRequest {
        request::DeleteHistoryItemsV1HistoryDeletePostRequest {
            http_client: &self,
            history_item_ids: history_item_ids.iter().map(|&x| x.to_owned()).collect(),
            xi_api_key: None,
        }
    }
    /**Delete History Item

    Delete a history item by its ID*/
    pub fn delete_history_item_v1_history_history_item_id_delete(
        &self,
        history_item_id: &str,
    ) -> request::DeleteHistoryItemV1HistoryHistoryItemIdDeleteRequest {
        request::DeleteHistoryItemV1HistoryHistoryItemIdDeleteRequest {
            http_client: &self,
            history_item_id: history_item_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Download History Items

    Download one or more history items. If one history item ID is provided, we will return a single audio file. If more than one history item IDs are provided, we will provide the history items packed into a .zip file.*/
    pub fn download_history_items_v1_history_download_post(
        &self,
        history_item_ids: &[&str],
    ) -> request::DownloadHistoryItemsV1HistoryDownloadPostRequest {
        request::DownloadHistoryItemsV1HistoryDownloadPostRequest {
            http_client: &self,
            history_item_ids: history_item_ids.iter().map(|&x| x.to_owned()).collect(),
            xi_api_key: None,
        }
    }
    /**Delete Sample

    Removes a sample by its ID.*/
    pub fn delete_sample_v1_voices_voice_id_samples_sample_id_delete(
        &self,
        sample_id: &str,
        voice_id: &str,
    ) -> request::DeleteSampleV1VoicesVoiceIdSamplesSampleIdDeleteRequest {
        request::DeleteSampleV1VoicesVoiceIdSamplesSampleIdDeleteRequest {
            http_client: &self,
            sample_id: sample_id.to_owned(),
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Get Audio From Sample

    Returns the audio corresponding to a sample attached to a voice.*/
    pub fn get_audio_from_sample_v1_voices_voice_id_samples_sample_id_audio_get(
        &self,
        sample_id: &str,
        voice_id: &str,
    ) -> request::GetAudioFromSampleV1VoicesVoiceIdSamplesSampleIdAudioGetRequest {
        request::GetAudioFromSampleV1VoicesVoiceIdSamplesSampleIdAudioGetRequest {
            http_client: &self,
            sample_id: sample_id.to_owned(),
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Text To Speech

    Converts text into speech using a voice of your choice and returns audio.*/
    pub fn text_to_speech_v1_text_to_speech_voice_id_post(
        &self,
        text: &str,
        voice_id: &str,
    ) -> request::TextToSpeechV1TextToSpeechVoiceIdPostRequest {
        request::TextToSpeechV1TextToSpeechVoiceIdPostRequest {
            http_client: &self,
            text: text.to_owned(),
            voice_id: voice_id.to_owned(),
            voice_settings: None,
            xi_api_key: None,
        }
    }
    /**Text To Speech

    Converts text into speech using a voice of your choice and returns audio as an audio stream.*/
    pub fn text_to_speech_v1_text_to_speech_voice_id_stream_post(
        &self,
        text: &str,
        voice_id: &str,
    ) -> request::TextToSpeechV1TextToSpeechVoiceIdStreamPostRequest {
        request::TextToSpeechV1TextToSpeechVoiceIdStreamPostRequest {
            http_client: &self,
            text: text.to_owned(),
            voice_id: voice_id.to_owned(),
            voice_settings: None,
            xi_api_key: None,
        }
    }
    /**Get User Subscription Info

    Gets extended information about the users subscription*/
    pub fn get_user_subscription_info_v1_user_subscription_get(
        &self,
    ) -> request::GetUserSubscriptionInfoV1UserSubscriptionGetRequest {
        request::GetUserSubscriptionInfoV1UserSubscriptionGetRequest {
            http_client: &self,
            xi_api_key: None,
        }
    }
    /**Get User Info

    Gets information about the user*/
    pub fn get_user_info_v1_user_get(&self) -> request::GetUserInfoV1UserGetRequest {
        request::GetUserInfoV1UserGetRequest {
            http_client: &self,
            xi_api_key: None,
        }
    }
    /**Get Voices

    Gets a list of all available voices for a user.*/
    pub fn get_voices_v1_voices_get(&self) -> request::GetVoicesV1VoicesGetRequest {
        request::GetVoicesV1VoicesGetRequest {
            http_client: &self,
            xi_api_key: None,
        }
    }
    /**Get Default Voice Settings.

    Gets the default settings for voices. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.*/
    pub fn get_default_voice_settings_v1_voices_settings_default_get(
        &self,
    ) -> request::GetDefaultVoiceSettingsV1VoicesSettingsDefaultGetRequest {
        request::GetDefaultVoiceSettingsV1VoicesSettingsDefaultGetRequest { http_client: &self }
    }
    /**Get Voice Settings

    Returns the settings for a specific voice. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.*/
    pub fn get_voice_settings_v1_voices_voice_id_settings_get(
        &self,
        voice_id: &str,
    ) -> request::GetVoiceSettingsV1VoicesVoiceIdSettingsGetRequest {
        request::GetVoiceSettingsV1VoicesVoiceIdSettingsGetRequest {
            http_client: &self,
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Get Voice

    Returns metadata about a specific voice.*/
    pub fn get_voice_v1_voices_voice_id_get(
        &self,
        voice_id: &str,
    ) -> request::GetVoiceV1VoicesVoiceIdGetRequest {
        request::GetVoiceV1VoicesVoiceIdGetRequest {
            http_client: &self,
            voice_id: voice_id.to_owned(),
            with_settings: None,
            xi_api_key: None,
        }
    }
    /**Delete Voice

    Deletes a voice by its ID.*/
    pub fn delete_voice_v1_voices_voice_id_delete(
        &self,
        voice_id: &str,
    ) -> request::DeleteVoiceV1VoicesVoiceIdDeleteRequest {
        request::DeleteVoiceV1VoicesVoiceIdDeleteRequest {
            http_client: &self,
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Edit Voice Settings

    Edit your settings for a specific voice. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.*/
    pub fn edit_voice_settings_v1_voices_voice_id_settings_edit_post(
        &self,
        similarity_boost: f64,
        stability: f64,
        voice_id: &str,
    ) -> request::EditVoiceSettingsV1VoicesVoiceIdSettingsEditPostRequest {
        request::EditVoiceSettingsV1VoicesVoiceIdSettingsEditPostRequest {
            http_client: &self,
            similarity_boost,
            stability,
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
    /**Add Voice

    Add a new voice to your collection of voices in VoiceLab.*/
    pub fn add_voice_v1_voices_add_post(&self) -> request::AddVoiceV1VoicesAddPostRequest {
        request::AddVoiceV1VoicesAddPostRequest {
            http_client: &self,
            xi_api_key: None,
        }
    }
    /**Edit Voice

    Edit a voice created by you.*/
    pub fn edit_voice_v1_voices_voice_id_edit_post(
        &self,
        voice_id: &str,
    ) -> request::EditVoiceV1VoicesVoiceIdEditPostRequest {
        request::EditVoiceV1VoicesVoiceIdEditPostRequest {
            http_client: &self,
            voice_id: voice_id.to_owned(),
            xi_api_key: None,
        }
    }
}

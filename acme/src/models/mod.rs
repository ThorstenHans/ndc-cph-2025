use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;

use crate::registrations::Registration;

#[derive(Debug, Serialize)]
pub struct SamplePayload {
    pub event: String,
    pub data: String,
}

impl IntoBody for SamplePayload {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct RegistrationRequestModel {
    pub url: String,
    pub event: String,
}

#[derive(Debug, Serialize)]
pub struct VerificationPayload {
    #[serde(rename = "keyData")]
    pub key_data: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponseModel {
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

impl IntoBody for RegistrationResponseModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl From<Registration> for RegistrationResponseModel {
    fn from(value: Registration) -> Self {
        Self {
            signing_key: value.signing_key.clone(),
        }
    }
}

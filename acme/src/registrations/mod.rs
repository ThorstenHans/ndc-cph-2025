use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{send, Method, RequestBuilder, Response},
    key_value::Store,
};
use uuid::Uuid;

use crate::models::VerificationPayload;

const KEY: &str = "ALL_REGISTRATIONS";
pub struct RegistrationService {}
impl RegistrationService {
    fn create_handshake_url(url: &String) -> String {
        format!("{}?handshake=true", url)
    }
    pub fn get_all() -> Result<Vec<Registration>> {
        let s = Store::open_default()?;
        let registrations = match s.get_json::<Vec<Registration>>(KEY)? {
            Some(r) => r,
            None => vec![],
        };
        Ok(registrations)
    }
    pub fn truncate() -> Result<()> {
        let s = Store::open_default()?;
        if s.exists(KEY)? {
            return s
                .delete(KEY)
                .with_context(|| "Erro truncating registrations");
        }
        Ok(())
    }

    pub async fn verify_registration_attempt(registration: &Registration) -> Result<()> {
        let verification_payload = serde_json::to_vec(&VerificationPayload {
            key_data: registration.signing_key.clone(),
        })?;
        let verification_request =
            RequestBuilder::new(Method::Post, Self::create_handshake_url(&registration.url))
                .header("Content-Type", "application/json")
                .body(verification_payload)
                .build();
        let verification_response: Response = send(verification_request).await?;
        match verification_response.status() {
            200 => Ok(()),
            _ => Err(Error::msg("Webhook validation request failed")),
        }
    }
    pub fn add(registration: &Registration) -> Result<()> {
        let store = Store::open_default()?;
        let mut registgrations = match store.get_json::<Vec<Registration>>(KEY)? {
            Some(v) => v,
            None => vec![],
        };
        registgrations.push(registration.clone());
        store.set_json(KEY, &registgrations)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub url: String,
    pub event: String,
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

impl Registration {
    pub fn new(url: String, event: String) -> Self {
        let key = Uuid::new_v4();
        Registration {
            url,
            event,
            signing_key: key.to_string(),
        }
    }
}

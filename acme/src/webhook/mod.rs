use anyhow::{Error, Result};
use spin_sdk::http::{conversions::IntoBody, Method, RequestBuilder, Response};

use crate::{fermyon::hmac::sign::sign, log, registrations::Registration};

pub async fn fire(registration: Registration, payload: impl IntoBody) -> Result<()> {
    let payload = payload.into_body();
    log(
        "DEMO",
        "Signing Webhook Payload With Registration Specific Key",
    );
    let signature = sign(&payload, &registration.signing_key.as_bytes())
        .map(|by| String::from_utf8(by).unwrap())
        .unwrap();

    log("DEMO", "Constructing outgoing HTTP request (POST)");
    let req = RequestBuilder::new(Method::Post, registration.url.clone())
        .header("X-Signature", signature)
        .body(payload)
        .build();

    log("DEMO", "Sending webhook to receiver");
    let response: Response = spin_sdk::http::send(req).await?;
    log(
        "DEMO",
        format!(
            "Received status {} from webhook receiver",
            response.status()
        )
        .as_str(),
    );
    match response.status() {
        200..299 => Ok(()),
        _ => Err(Error::msg(
            "Received invalid response from Webhook Receiver",
        )),
    }
}

use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

use crate::fermyon::hmac::verify::verify;

wit_bindgen::generate!({
    world: "consumer",
    path: "../wit"
});

#[derive(Debug, Deserialize)]
pub struct HandshakeRequestModel {
    #[serde(rename = "keyData")]
    key_data: String,
}

#[http_component]
fn handle_consumer(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/target", handle_invocation);
    Ok(router.handle(req))
}

fn handle_invocation(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let r = match is_handshake(&req) {
        true => handle_hanshake(&req, params),
        false => handle_webhook_invocation(&req, params),
    };
    Ok(r)
}

fn is_handshake(req: &Request) -> bool {
    req.query().contains("handshake=true")
}

const KEY: &str = "KEYS";

fn handle_hanshake(req: &Request, _: Params) -> anyhow::Result<Response> {
    log(
        "HANDSHAKE",
        "Webhook producer sent a verifcation request (handshake)",
    );
    let Ok(model) = serde_json::from_slice::<HandshakeRequestModel>(req.body()) else {
        log("HANDSHAKE", "Could not deserialize verification payload");
        return Ok(Response::new(400, ()));
    };
    log(
        "HANDSHAKE",
        "Valid verification payload received, storing signing key",
    );
    let s = Store::open_default()?;
    Ok(match s.set(KEY, model.key_data.as_bytes()) {
        Ok(_) => {
            log("HANDSHAKE", "Done");
            Response::new(200, ())
        }
        Err(e) => {
            log(
                "HANDSHAKE",
                format!("Error while storing signing key {:?}", e).as_str(),
            );
            Response::new(500, ())
        }
    })
}

fn handle_webhook_invocation(req: &Request, _: Params) -> anyhow::Result<Response> {
    log("INCOMING WEBHOOK", "Received Incoming Webhook");
    let s = Store::open_default()?;
    let Some(key) = s.get(KEY)? else {
        log("INCOMING WEBHOOK", "Error retrieving signing key");
        return Ok(Response::new(500, ()));
    };

    log("INCOMING WEBHOOK", "Signing Key loaded");

    let body = req.body();
    // todo! refactor
    log(
        "INCOMING WEBHOOK",
        "Grabbing Signature from incoming webhook headers",
    );
    let signature = req.header("X-Signature").unwrap().as_bytes();
    log("INCOMING WEBHOOK", "Verifying payload integrity");
    match verify(body, key.as_slice(), signature) {
        true => {
            log(
                "INCOMING WEBHOOK",
                "Payload Verification Succeeded. Payload is GOOD!",
            );
            log("INCOMING WEBHOOK", "DONE");
            Ok(Response::new(200, ()))
        }
        false => {
            log(
                "INCOMING WEBHOOK",
                "Payload Verification Failed. Payload may have been mutated in transit (BAD)",
            );
            Ok(Response::new(500, ()))
        }
    }
}

pub fn log(scope: &str, message: &str) {
    println!("[{scope}]: {message}");
}

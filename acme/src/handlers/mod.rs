use anyhow::Result;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder};

use crate::{
    log,
    models::{RegistrationRequestModel, RegistrationResponseModel, SamplePayload},
    registrations::{Registration, RegistrationService},
    webhook,
};

pub(crate) fn dump_registrations(_: Request, _: Params) -> Result<impl IntoResponse> {
    log("Dump Registrations", "All registrations requested");
    let registrations = RegistrationService::get_all()?;
    let payload = serde_json::to_vec(&registrations)?;

    Ok(ResponseBuilder::new(200)
        .header("content-type", "application/json")
        .body(payload)
        .build())
}

pub(crate) async fn register_webhook(req: Request, _: Params) -> Result<impl IntoResponse> {
    log("Register Webhook", "New webhook registration requested");
    let Ok(model) = serde_json::from_slice::<RegistrationRequestModel>(req.body()) else {
        return Ok(Response::new(400, "Invalid Payload received"));
    };
    log("Register Webhook", "Received valid registration payload");

    let registration = Registration::new(model.url, model.event);
    log("Register Webhook", "Issuing Verification request");
    match RegistrationService::verify_registration_attempt(&registration)
        .await
        .and_then(|_| RegistrationService::add(&registration))
    {
        Ok(_) => {
            log(
                "Register Webhook",
                "Registration verified. Will exchange signing key now",
            );
            Ok(ResponseBuilder::new(200)
                .header("content-type", "application/json")
                .body(RegistrationResponseModel::from(registration))
                .build())
        }
        _ => {
            log(
                "Register Webhook",
                "Verification failed - Registration not accepted",
            );
            Ok(Response::new(500, ()))
        }
    }
}

pub(crate) fn truncate_registrations(_: Request, _: Params) -> Result<impl IntoResponse> {
    log("Truncate Registrations", "Will truncrate all registrations");
    RegistrationService::truncate()?;

    Ok(Response::new(204, ()))
}

pub(crate) async fn demonstrate_fire_webhook(_: Request, _: Params) -> Result<impl IntoResponse> {
    log("DEMO", "Loading registered and verified webhook receivers");
    let registrations = RegistrationService::get_all()?;
    log(
        "DEMO",
        format!("Found {} registrations", registrations.len()).as_str(),
    );
    for reg in registrations.into_iter() {
        let payload = SamplePayload {
            event: reg.event.clone(),
            data: reg.url.clone(),
        };
        match webhook::fire(reg, payload).await {
            Ok(_) => (),
            Err(_) => log("DEMO", "Receiver responded with an error :("),
        };
    }
    Ok(Response::new(200, ()))
}

use crate::handlers::{
    demonstrate_fire_webhook, dump_registrations, register_webhook, truncate_registrations,
};
use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;
mod handlers;
mod models;
mod registrations;
mod webhook;

wit_bindgen::generate!({
    world:"producer",
    path: "../wit/world.wit",
    generate_all,
});
#[http_component]
fn handle_simple_http_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post_async("/registrations", register_webhook);
    router.get("/registrations", dump_registrations);
    router.delete("/registrations", truncate_registrations);
    router.post_async("/fire", demonstrate_fire_webhook);
    Ok(router.handle(req))
}

pub fn log(scope: &str, message: &str) {
    println!("[{scope}]: {message}");
}

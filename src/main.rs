#![feature(async_closure, let_chains)]

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
mod client;
#[cfg(target_arch = "wasm32")]
use engine::application::bus::BrowserBus;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn init_main(
  id: String,
  assets_location: String,
  bus: BrowserBus,
  session_id: String,
  access_token: String,
  udp_url: String,
  tcp_url: String,
) {
  client::main(
    id,
    assets_location,
    bus,
    session_id,
    access_token,
    udp_url,
    tcp_url,
  )
  .await;
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() {
  server::main().await;
}

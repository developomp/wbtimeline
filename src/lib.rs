#![recursion_limit = "512"]

mod app;
mod components;
mod state;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();

    Ok(())
}

// Copyright © 2020 Lukas Wagner

#![recursion_limit = "512"]

macro_rules! println {
    ($($tt:tt)*) => {{
        let msg = format!($($tt)*);
        js! { console.log(@{ msg }) }
    }}
}

mod app;
mod utils;

use seed::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::init();
    seed::App::builder(app::update, app::view).build_and_start();
    Ok(())
}

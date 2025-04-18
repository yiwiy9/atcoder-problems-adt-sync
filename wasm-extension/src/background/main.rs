use crate::background::chrome_message;
use crate::log;
use wasm_bindgen::prelude::*;

/// Entry point for the background script.
/// Called from JavaScript instead of using `#[wasm_bindgen(start)]`
/// because the extension has multiple entry points.
#[wasm_bindgen]
pub fn run_background() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    log::info("Background script start");

    chrome_message::register_message_listener();

    log::info("Background script end");

    Ok(())
}

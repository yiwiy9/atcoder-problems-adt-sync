use crate::content::service::cleanup;
use crate::content::ui::initialize_ui;
use crate::log;
use wasm_bindgen::prelude::*;

/// Entry point for the content script.
/// Called from JavaScript instead of using `#[wasm_bindgen(start)]`
/// because the extension has multiple entry points.
#[wasm_bindgen]
pub async fn run_content() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log::info("Content script start");

    cleanup(vec![]).await?;

    initialize_ui().await?;

    log::info("Content script end");

    Ok(())
}

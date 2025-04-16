use crate::bindings::chrome_storage;
use wasm_bindgen::prelude::*;

const KEY: &str = "sync_enabled";

pub async fn get_sync_enabled() -> Result<bool, JsValue> {
    let value = chrome_storage::get_local(KEY).await?;
    Ok(value.unwrap_or(JsValue::FALSE).as_bool().unwrap_or(false))
}

pub async fn set_sync_enabled(enabled: bool) -> Result<(), JsValue> {
    chrome_storage::set_local(KEY, &JsValue::from_bool(enabled)).await?;
    Ok(())
}

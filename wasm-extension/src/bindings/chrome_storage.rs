use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

#[wasm_bindgen(module = "/extension/js/chrome_storage.js")]
extern "C" {
    #[wasm_bindgen(js_name = getLocal)]
    fn get_local_js(key: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = setLocal)]
    fn set_local_js(key: &str, value: &JsValue) -> js_sys::Promise;
}

pub async fn get_local(key: &str) -> Result<Option<JsValue>, JsValue> {
    let result = JsFuture::from(get_local_js(key)).await?;
    Ok(if result.is_undefined() {
        None
    } else {
        Some(result)
    })
}

pub async fn set_local(key: &str, value: &JsValue) -> Result<(), JsValue> {
    JsFuture::from(set_local_js(key, value)).await?;
    Ok(())
}

use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

pub const API_BASE_URL: &str = "https://api.example.com";

/// Fetches a JSON response from the given URL and deserializes it into type `T`.
pub async fn fetch_json<T>(url: &str) -> Result<T, JsValue>
where
    T: DeserializeOwned,
{
    let options = web_sys::RequestInit::new();
    options.set_method("GET");
    options.set_mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(url, &options)?;

    // Use `global()` to support environments like Service Workers where `window` is not available.
    let fetch_fn = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("fetch"))?;
    let fetch = fetch_fn
        .dyn_into::<js_sys::Function>()
        .map_err(|_| JsValue::from_str("Failed to get fetch function from global object"))?;

    let promise = fetch.call1(&JsValue::null(), &request)?;
    let response = JsFuture::from(js_sys::Promise::from(promise))
        .await?
        .dyn_into::<web_sys::Response>()?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let json = JsFuture::from(response.json()?).await?;
    let data: T = serde_wasm_bindgen::from_value(json)?;

    Ok(data)
}

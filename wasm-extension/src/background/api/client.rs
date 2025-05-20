use crate::background::api::constants::{API_BASE_URL, EXTENSION_NAME, X_EXTENSION_NAME_HEADER};
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

/// Sends a Request and returns the parsed JSON response as type `T`.
/// Used as a low-level helper by higher-level API fetch functions.
async fn fetch_json<T>(request: &web_sys::Request) -> Result<T, JsValue>
where
    T: DeserializeOwned,
{
    // Use `global()` to support environments like Service Workers where `window` is not available.
    let fetch_fn = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("fetch"))?;
    let fetch = fetch_fn
        .dyn_into::<js_sys::Function>()
        .map_err(|_| JsValue::from_str("Failed to get fetch function from global object"))?;

    let promise = fetch.call1(&JsValue::null(), request)?;
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

/// Sends a GET request to the API base URL with authorization header,
/// and parses the JSON response into `T`.
pub async fn fetch_api_json<T>(path: &str) -> Result<T, JsValue>
where
    T: DeserializeOwned,
{
    // Ensure path starts with '/'
    let normalized_path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };
    let url = format!("{}{}", API_BASE_URL, normalized_path);

    let options = web_sys::RequestInit::new();
    options.set_method("GET");
    options.set_mode(web_sys::RequestMode::Cors);

    let headers = web_sys::Headers::new()?;
    headers.set(X_EXTENSION_NAME_HEADER, EXTENSION_NAME)?;
    headers.set("Content-Type", "application/json")?;
    options.set_headers(&headers);

    let request = web_sys::Request::new_with_str_and_init(&url, &options)?;
    fetch_json::<T>(&request).await
}

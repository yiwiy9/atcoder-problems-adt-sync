use crate::log;
use gloo::timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use web_sys::js_sys;

const RETRY_COUNT: u32 = 3;
const RETRY_DELAY_MS: u64 = 200;

#[wasm_bindgen(module = "/extension/js/chrome_message.js")]
extern "C" {
    #[wasm_bindgen(js_name = sendMessage)]
    fn send_message_js(message: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(js_name = registerAsyncMessageHandler)]
    fn register_async_message_handler_js(async_handler: &js_sys::Function);
}

/// Sends a message to the background script, retrying on transient connection errors.
/// Especially useful when the service worker is temporarily inactive (e.g. after tab reactivation).
pub async fn send_message(message: &JsValue) -> Result<JsValue, JsValue> {
    for attempt in 0..RETRY_COUNT {
        let result = JsFuture::from(send_message_js(message)).await;

        match result {
            Ok(response) => return Ok(response),
            Err(err) => {
                if !is_retry_error(&err) || attempt == RETRY_COUNT - 1 {
                    return Err(err);
                }

                log::debug(format!(
                    "Attempt {} failed. Retrying in {} ms...",
                    attempt + 1,
                    RETRY_DELAY_MS
                ));

                sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
        }
    }

    Err(JsValue::from_str("Failed to send message after retries"))
}

/// Registers an async message handler that responds via a Promise.
pub fn register_async_message_handler<F, Fut>(handler: F)
where
    F: Fn(JsValue, JsValue) -> Fut + 'static,
    Fut: std::future::Future<Output = Result<JsValue, JsValue>> + 'static,
{
    let closure = Closure::wrap(Box::new(
        move |message: JsValue, sender: JsValue| -> js_sys::Promise {
            future_to_promise(handler(message, sender))
        },
    ) as Box<dyn Fn(JsValue, JsValue) -> js_sys::Promise>);

    register_async_message_handler_js(closure.into_js_value().unchecked_ref());
}

/// Determines if the given JS error is a transient connection issue that can be retried.
fn is_retry_error(js_error: &JsValue) -> bool {
    if let Some(message) = js_error.as_string() {
        return is_retry_message(&message);
    }

    if js_error.is_object() {
        if let Ok(message_val) = js_sys::Reflect::get(js_error, &JsValue::from_str("message")) {
            if let Some(message) = message_val.as_string() {
                return is_retry_message(&message);
            }
        }
    }

    false
}

/// Matches error messages known to indicate recoverable connection issues.
fn is_retry_message(message: &str) -> bool {
    message.contains("Could not establish connection")
        || message.contains("Receiving end does not exist")
        || message.contains("The message port closed before a response was received")
}

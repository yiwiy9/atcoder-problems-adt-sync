use wasm_bindgen::JsValue;
use web_sys::{console, js_sys};

/// Logs an debug message with a standardized prefix.
pub fn debug(message: impl Into<JsValue>) {
    log_with_prefix("[ADT Sync]", message.into(), console::debug);
}

/// Logs an info message with a standardized prefix.
pub fn info(message: impl Into<JsValue>) {
    log_with_prefix("[ADT Sync]", message.into(), console::log);
}

/// Logs a warning message with a standardized prefix.
pub fn warn(message: impl Into<JsValue>) {
    log_with_prefix("[ADT Sync]", message.into(), console::warn);
}

/// Logs an error message with a standardized prefix.
pub fn error(message: impl Into<JsValue>) {
    log_with_prefix("[ADT Sync]", message.into(), console::error);
}

/// Internal helper to add prefix and log structured messages.
fn log_with_prefix<F>(prefix: &str, value: JsValue, log_fn: F)
where
    F: Fn(&js_sys::Array),
{
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(prefix));
    array.push(&value);
    log_fn(&array);
}

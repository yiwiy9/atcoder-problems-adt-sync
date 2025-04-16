use crate::content::ui::utils;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

/// Global flag to mark if the next hashchange event should be ignored.
/// This is set to true right before programmatically modifying the hash,
/// and reset immediately after the event is triggered.
static IGNORE_NEXT_HASH_CHANGE: OnceLock<AtomicBool> = OnceLock::new();

/// Initializes the global AtomicBool (only once).
fn ignore_flag() -> &'static AtomicBool {
    IGNORE_NEXT_HASH_CHANGE.get_or_init(|| AtomicBool::new(false))
}

/// Marks that the next hashchange event should be ignored.
pub fn mark_ignore_next_hash_change() {
    ignore_flag().store(true, Ordering::SeqCst);
}

/// Checks whether the next hashchange should be ignored,
/// and resets the flag to false.
fn should_ignore_hash_change() -> bool {
    ignore_flag().swap(false, Ordering::SeqCst)
}

/// Registers a listener for URL hash changes and triggers a sync task
/// based on the current user ID extracted from the URL.
/// This is invoked on navigation or input-based hash updates.
pub fn register_hash_change_listener() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))?;

    let closure = Closure::wrap(Box::new(move || {
        // Skip if this was a programmatic hash change
        if should_ignore_hash_change() {
            return;
        }
        utils::spawn_sync_from_url();
    }) as Box<dyn Fn()>);

    window.set_onhashchange(Some(closure.into_js_value().unchecked_ref()));
    Ok(())
}

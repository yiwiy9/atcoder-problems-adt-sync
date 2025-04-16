use crate::content::ui::constants::REACT_RERENDER_WAIT_MS;
use crate::content::ui::toggle;
use crate::content::ui::url;
use crate::log;
use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;

const HASH_RESTORE_DELAY_MS: u32 = 50;

/// Temporarily modifies the URL to trigger React Router to refresh hooks like useSWR,
/// then restores the original path after a short delay.
/// This is useful for forcing re-fetch when IndexedDB data is added dynamically.
pub fn trigger_react_refresh() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))?;
    let location = window.location();

    let original_hash = location.hash()?;
    let mut segments = original_hash
        .trim_start_matches('#')
        .split('/')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Only trigger refresh on /#/table/{user_id}/... pages
    if segments.get(1) != Some(&"table".into()) || segments.get(2).is_none_or(|s| s.is_empty()) {
        return Err(JsValue::from_str(
            "Not on a valid /#/table/{user_id} page; skipping refresh",
        ));
    }

    log::debug("React refresh triggered via hash rewrite");

    // Disable toggle briefly to prevent user interactions while refreshing.
    toggle::disable_toggle_temporarily(REACT_RERENDER_WAIT_MS)?;

    // Mark this change as programmatic
    url::mark_ignore_next_hash_change();

    // Note: We avoid appending to the rival_id segment because when it is originally empty,
    // the React-controlled input field retains the dummy value after restoration.
    // Since the user_id is always present when this function is called,
    // we safely append an underscore to user_id instead.
    segments[2] = format!("{}_", segments[2]);

    let temp_hash = format!("#{}", segments.join("/"));
    location.set_hash(&temp_hash)?;

    Timeout::new(HASH_RESTORE_DELAY_MS, move || {
        url::mark_ignore_next_hash_change();
        let window = web_sys::window().unwrap();
        let _ = window.location().set_hash(&original_hash);
        log::info("React refresh completed");
    })
    .forget();

    Ok(())
}

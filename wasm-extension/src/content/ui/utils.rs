use crate::content::chrome_storage;
use crate::content::service;
use crate::content::ui::constants::REACT_RERENDER_WAIT_MS;
use crate::content::ui::refresh;
use crate::log;
use gloo::timers::callback::Timeout;

/// Returns true if the current URL hash represents a `/#/table` page,
/// regardless of whether a user ID is present.
pub fn is_table_page() -> bool {
    let hash = match web_sys::window().and_then(|w| w.location().hash().ok()) {
        Some(hash) => hash,
        None => return false,
    };
    let segments = hash.trim_start_matches('#').split('/').collect::<Vec<_>>();
    matches!(segments.get(1), Some(&"table"))
}

/// Extracts the user ID from the current page URL (e.g., "/#/table/{user_id}/...").
/// Returns `None` if the user ID is not found or the URL format is unexpected.
pub fn get_user_id_from_url() -> Option<String> {
    let location = web_sys::window()?.location();
    let hash = location.hash().ok()?; // e.g., "#/table/user1/rival1"

    let mut segments = hash.trim_start_matches('#').split('/');

    // Expecting something like: ["", "table", "user_id", ...]
    match (segments.nth(1), segments.next()) {
        (Some("table"), Some(user_id)) if !user_id.is_empty() => Some(user_id.to_string()),
        _ => None,
    }
}

/// Spawns a background task to sync data based on the current URL.
/// If a user ID exists, it performs sync and triggers refresh.
/// Otherwise, it performs cleanup.
pub fn spawn_sync_from_url() {
    wasm_bindgen_futures::spawn_local(async {
        let enabled = match chrome_storage::get_sync_enabled().await {
            Ok(enabled) => enabled,
            Err(err) => {
                log::error(&err);
                return;
            }
        };
        if !enabled {
            return;
        }

        match get_user_id_from_url() {
            Some(user_id) => {
                if let Err(err) = service::sync_and_cleanup(&user_id).await {
                    log::error(&err);
                    return;
                }

                // Delay to let React rerender first
                Timeout::new(REACT_RERENDER_WAIT_MS, move || {
                    if let Err(err) = refresh::trigger_react_refresh() {
                        log::error(&err);
                    }
                })
                .forget();
            }
            None => {
                if let Err(err) = service::cleanup(vec![]).await {
                    log::error(&err);
                }
            }
        }
    });
}

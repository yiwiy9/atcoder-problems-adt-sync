use crate::content::ui::constants::REACT_RERENDER_WAIT_MS;
use crate::content::ui::toggle;
use crate::content::ui::toggle_observer;
use crate::content::ui::url;
use crate::content::ui::utils;
use crate::content::ui::visibility;
use wasm_bindgen::prelude::*;

/// Initialize UI components such as the ADT toggle.
/// This should be called once on page load.
pub async fn initialize_ui() -> Result<(), JsValue> {
    // Watch for DOM changes to re-insert the toggle if needed (e.g. after SPA navigation)
    toggle_observer::observe_and_insert_toggle()?;

    // Listen for URL hash changes and trigger sync if needed
    url::register_hash_change_listener()?;

    // Listen for tab visibility change and trigger sync/cleanup accordingly
    visibility::register_visibility_change_listener()?;

    // Only insert the toggle and apply refresh protection if on table page
    if utils::is_table_page() {
        toggle::insert_toggle_button().await?;
        toggle::disable_toggle_temporarily(REACT_RERENDER_WAIT_MS)?;
    }

    Ok(())
}

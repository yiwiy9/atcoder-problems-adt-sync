use crate::content::ui::toggle;
use crate::content::ui::utils;
use wasm_bindgen::prelude::*;

/// Observes DOM changes and inserts the ADT toggle on the table page if missing.
/// Ensures the toggle is re-inserted after SPA navigation.
pub fn observe_and_insert_toggle() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or_else(|| JsValue::from_str("No global window exists"))?
        .document()
        .ok_or_else(|| JsValue::from_str("No document exists"))?;

    let body = document
        .body()
        .ok_or_else(|| JsValue::from_str("No body element exists"))?;

    let closure = Closure::wrap(Box::new(move || {
        if !utils::is_table_page() {
            return;
        }
        if document
            .get_element_by_id(toggle::ADT_TOGGLE_CONTAINER_ID)
            .is_some()
        {
            return;
        }
        wasm_bindgen_futures::spawn_local(async {
            let _ = toggle::insert_toggle_button().await;
        });
    }) as Box<dyn Fn()>);

    let observer = web_sys::MutationObserver::new(closure.into_js_value().unchecked_ref())?;

    let option = web_sys::MutationObserverInit::new();
    option.set_subtree(true);
    option.set_child_list(true);

    observer.observe_with_options(&body, &option)?;

    Ok(())
}

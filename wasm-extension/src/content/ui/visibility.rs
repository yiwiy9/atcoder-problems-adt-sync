use crate::content::service;
use crate::content::ui::utils;
use crate::log;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Registers a listener for visibility change events.
/// When the tab becomes hidden, it performs a cleanup task.
/// When the tab becomes visible, it performs a sync task.
pub fn register_visibility_change_listener() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or_else(|| JsValue::from_str("No global window exists"))?
        .document()
        .ok_or_else(|| JsValue::from_str("No document exists"))?;

    let document = Rc::new(document);

    let closure = {
        let document = Rc::clone(&document);
        Closure::wrap(Box::new(move || {
            if document.hidden() {
                wasm_bindgen_futures::spawn_local(async {
                    if let Err(err) = service::cleanup(vec![]).await {
                        log::error(&err);
                    }
                });
            } else {
                utils::spawn_sync_from_url();
            }
        }) as Box<dyn Fn()>)
    };

    document.set_onvisibilitychange(Some(closure.into_js_value().unchecked_ref()));
    Ok(())
}

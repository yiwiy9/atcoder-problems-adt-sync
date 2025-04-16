use crate::content::chrome_storage;
use crate::content::service;
use crate::content::ui::refresh;
use crate::content::ui::utils;
use crate::log;
use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;

pub const ADT_TOGGLE_CONTAINER_ID: &str = "adt-toggle";
const ADT_TOGGLE_INPUT_ID: &str = "sync-adt-submissions";
const ADT_TOGGLE_LABEL: &str = "Sync ADT Submissions";

/// Insert a toggle button to enable/disable syncing ADT submissions
pub async fn insert_toggle_button() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document exists"))?;

    // Prevent duplicate insertion
    if document
        .get_element_by_id(ADT_TOGGLE_CONTAINER_ID)
        .is_some()
    {
        return Ok(());
    }

    let container_elements = document.get_elements_by_class_name("form-check form-check-inline");

    if container_elements.length() == 0 {
        return Err(JsValue::from_str("No toggle elements found"));
    }

    let last_container_element = container_elements
        .item(container_elements.length() - 1)
        .ok_or_else(|| JsValue::from_str("Failed to get last toggle container"))?;

    // Clone the last container element to create the new toggle button
    let adt_container_element = last_container_element
        .clone_node_with_deep(true)?
        .dyn_into::<web_sys::Element>()?;

    let adt_input_element = adt_container_element
        .query_selector("input.custom-control-input")?
        .ok_or_else(|| JsValue::from_str("Failed to get input element in toggle container"))?
        .dyn_into::<web_sys::HtmlInputElement>()?;

    let adt_label_element = adt_container_element
        .query_selector("label.custom-control-label")?
        .ok_or_else(|| JsValue::from_str("Failed to get label element in toggle container"))?
        .dyn_into::<web_sys::HtmlLabelElement>()?;

    // Modify the cloned elements to create the new toggle button
    adt_container_element.set_id(ADT_TOGGLE_CONTAINER_ID);
    adt_input_element.set_id(ADT_TOGGLE_INPUT_ID);
    adt_label_element.set_html_for(ADT_TOGGLE_INPUT_ID);
    adt_label_element.set_text_content(Some(ADT_TOGGLE_LABEL));

    set_init_toggle_state(&adt_input_element).await?;
    add_toggle_listener(&adt_input_element)?;

    last_container_element.after_with_node_1(&adt_container_element)?;

    log::info("Toggle button inserted successfully");
    Ok(())
}

/// Set the initial state of the toggle button based on the sync state
async fn set_init_toggle_state(toggle: &web_sys::HtmlInputElement) -> Result<(), JsValue> {
    let sync_enabled = chrome_storage::get_sync_enabled().await?;

    let final_state = if sync_enabled {
        match utils::get_user_id_from_url() {
            Some(user_id) => match service::sync_and_cleanup(&user_id).await {
                Ok(_) => {
                    refresh::trigger_react_refresh().ok();
                    true
                }
                Err(err) => {
                    log::error(&err);
                    chrome_storage::set_sync_enabled(false).await?;
                    false
                }
            },
            None => true,
        }
    } else {
        false
    };

    toggle.set_checked(final_state);
    Ok(())
}

/// Add a listener to the toggle button to sync ADT submissions when toggled
fn add_toggle_listener(toggle: &web_sys::HtmlInputElement) -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        // Use `spawn_local` to execute async code in a non-async closure
        wasm_bindgen_futures::spawn_local(async move {
            let Some(toggle) = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            else {
                log::error("Failed to get toggle element from event");
                return;
            };

            let checked = toggle.checked();

            if let Err(err) = chrome_storage::set_sync_enabled(checked).await {
                log::error(&err);
                return;
            }

            if checked {
                let Some(user_id) = utils::get_user_id_from_url() else {
                    return;
                };
                if let Err(err) = service::sync_and_cleanup(&user_id).await {
                    log::error(&err);
                    return;
                }
            } else if let Err(err) = service::cleanup(vec![]).await {
                log::error(&err);
                return;
            }

            if let Err(err) = refresh::trigger_react_refresh() {
                log::error(&err);
            }
        });
    }) as Box<dyn Fn(_)>);

    toggle.add_event_listener_with_callback("change", closure.into_js_value().unchecked_ref())?;
    Ok(())
}

/// Disable the ADT toggle temporarily (e.g. during refresh)
pub fn disable_toggle_temporarily(duration_ms: u32) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document exists"))?;

    let adt_input_element = document
        .get_element_by_id(ADT_TOGGLE_INPUT_ID)
        .ok_or_else(|| JsValue::from_str("No toggle element exists"))?
        .dyn_into::<web_sys::HtmlInputElement>()?;

    adt_input_element.set_disabled(true);

    Timeout::new(duration_ms, move || {
        adt_input_element.set_disabled(false);
    })
    .forget();

    Ok(())
}

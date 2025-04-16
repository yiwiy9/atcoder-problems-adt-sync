use crate::background::use_case::get_ac_problem_ids;
use crate::bindings::chrome_message::register_async_message_handler;
use crate::messages::{Request, Response};
use wasm_bindgen::prelude::*;

pub fn register_message_listener() {
    register_async_message_handler(handle_message);
}

async fn handle_message(message: JsValue, _sender: JsValue) -> Result<JsValue, JsValue> {
    let request: Request = match serde_wasm_bindgen::from_value(message) {
        Ok(r) => r,
        Err(err) => {
            let response = Response::Error {
                message: format!("Invalid request format: {:?}", err),
            };
            return serde_wasm_bindgen::to_value(&response).map_err(JsValue::from);
        }
    };

    let response = match request {
        Request::GetAcProblemIds { user_id } => match get_ac_problem_ids::handle(user_id).await {
            Ok(problem_ids) => Response::GetProblemIds { problem_ids },
            Err(err) => Response::Error {
                message: format!("GetAcProblemIds failed: {:?}", err),
            },
        },
    };

    serde_wasm_bindgen::to_value(&response).map_err(JsValue::from)
}

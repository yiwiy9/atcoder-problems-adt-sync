use crate::bindings::chrome_message;
use crate::messages::{Request, Response};
use crate::models::{Problem, User};
use wasm_bindgen::prelude::*;

async fn get_ac_problem_ids(user_id: &str) -> Result<Vec<String>, JsValue> {
    let request = Request::GetAcProblemIds {
        user_id: user_id.to_string(),
    };

    let message = serde_wasm_bindgen::to_value(&request)?;
    let result = chrome_message::send_message(&message).await?;

    let response: Response = serde_wasm_bindgen::from_value(result)?;

    match response {
        Response::GetProblemIds { problem_ids } => Ok(problem_ids),
        Response::Error { message } => Err(JsValue::from_str(&message)),
    }
}

pub async fn get_problems_by_user(user: &User) -> Result<Vec<Problem>, JsValue> {
    let problem_ids = get_ac_problem_ids(&user.id).await?;

    let problems = problem_ids
        .iter()
        .filter_map(|id| Problem::new(id).ok())
        .collect();

    Ok(problems)
}

use crate::background::api::client::fetch_api_json;
use crate::log;
use crate::models::{Problem, User};
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Deserialize)]
struct Response {
    problem_ids: Vec<String>,
}

pub async fn fetch_ac_problems(user: &User) -> Result<Vec<Problem>, JsValue> {
    let path = format!("/users/{}/problems", user.id);
    let response: Response = fetch_api_json(&path).await?;

    let mut problems = Vec::new();
    for problem_id in response.problem_ids {
        match Problem::new(&problem_id) {
            Ok(problem) => problems.push(problem),
            Err(_) => log::warn(format!("Failed to create problem from ID: {}", problem_id)),
        }
    }

    Ok(problems)
}

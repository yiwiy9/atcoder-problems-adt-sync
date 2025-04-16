use crate::background::api::client::{API_BASE_URL, fetch_json};
use crate::log;
use crate::models::{Problem, User};
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Deserialize)]
struct Response {
    problem_ids: Vec<String>,
}

pub async fn fetch_ac_problems(user: &User) -> Result<Vec<Problem>, JsValue> {
    // TODO: Remove mock when backend is ready
    if let Some(mocked) = get_mock_problems(&user.id) {
        return Ok(mocked);
    }

    let url = format!("{}/users/{}/problems", API_BASE_URL, user.id);
    let response: Response = fetch_json(&url).await?;

    let mut problems = Vec::new();
    for problem_id in response.problem_ids {
        match Problem::new(&problem_id) {
            Ok(problem) => problems.push(problem),
            Err(_) => log::warn(format!("Failed to create problem from ID: {}", problem_id)),
        }
    }

    Ok(problems)
}

fn get_mock_problems(user_id: &str) -> Option<Vec<Problem>> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let hash = {
        let mut hasher = DefaultHasher::new();
        user_id.hash(&mut hasher);
        hasher.finish() % 5
    };

    let ids = match hash {
        0 => vec!["abc100_a", "abc100_b", "abc100_c"],
        1 => vec!["abc135_a", "abc135_b", "abc135_c"],
        2 => vec!["abc196_a", "abc196_b", "abc196_c"],
        3 => vec!["abc250_a", "abc250_b", "abc250_c"],
        4 => vec!["abc338_a", "abc338_b", "abc338_c"],
        _ => return None,
    };

    let problems: Vec<Problem> = ids
        .into_iter()
        .filter_map(|id| Problem::new(id).ok())
        .collect();

    Some(problems)
}

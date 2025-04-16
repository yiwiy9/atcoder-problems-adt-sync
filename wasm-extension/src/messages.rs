use serde::{Deserialize, Serialize};

/// Request message types from content to background script
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Request {
    GetAcProblemIds { user_id: String },
}

/// Response message types from background to content script
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Response {
    GetProblemIds { problem_ids: Vec<String> },
    Error { message: String },
}

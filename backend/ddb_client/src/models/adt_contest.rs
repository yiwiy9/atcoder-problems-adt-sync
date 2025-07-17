use crate::models::traits::ToWriteRequest;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

/// Represents a single ADT contest record stored in DynamoDB.
/// PK: "CONTEST", SK: "{start_epoch_second}-{difficulty_order}"
#[derive(Debug, Serialize, Deserialize)]
pub struct AdtContestRecord {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub contest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetched_submission_id: Option<u64>,
}

impl AdtContestRecord {
    /// Return the partition key (PK) AttributeValue.
    pub fn pk_attr() -> AttributeValue {
        AttributeValue::S("CONTEST".to_string())
    }

    /// Generate the sort key (SK) AttributeValue based on the contest start time.
    pub fn sk_attr(start_epoch_second: u64, contest_id: &str) -> AttributeValue {
        let order = Self::difficulty_order(contest_id);
        let sk = format!("{:010}-{:02}", start_epoch_second, order);
        AttributeValue::S(sk)
    }

    pub fn start_epoch_second(&self) -> u64 {
        self.sk
            .split('-')
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
    }

    fn difficulty_order(contest_id: &str) -> u8 {
        if contest_id.contains("_easy") {
            1
        } else if contest_id.contains("_medium") {
            2
        } else if contest_id.contains("_hard") {
            3
        } else {
            4 // assume "_all" or default
        }
    }
}

impl ToWriteRequest for AdtContestRecord {}

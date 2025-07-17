use crate::models::traits::ToWriteRequest;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

/// Represents a user's AC problem list stored in DynamoDB.
/// PK format: "USER_AC#{user_id}", SK: "AC"
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAcProblemRecord {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub ac_problems: Vec<String>,
}

impl UserAcProblemRecord {
    /// Generate the partition key (PK) AttributeValue for a given user ID.
    pub fn pk_attr(user_id: &str) -> AttributeValue {
        AttributeValue::S(format!("USER_AC#{}", user_id))
    }

    /// Return the fixed sort key (SK) AttributeValue.
    pub fn sk_attr() -> AttributeValue {
        AttributeValue::S("AC".to_string())
    }

    pub fn user_id(&self) -> String {
        self.pk.strip_prefix("USER_AC#").unwrap_or("").to_string()
    }

    /// Merge accepted problems from another record into this one.
    /// The resulting list will be sorted and deduplicated.
    pub fn merge_ac_problems_from(&mut self, other: &Self) {
        self.ac_problems.extend(other.ac_problems.iter().cloned());
        self.ac_problems.sort();
        self.ac_problems.dedup();
    }
}

impl ToWriteRequest for UserAcProblemRecord {}

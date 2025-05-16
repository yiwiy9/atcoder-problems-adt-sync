use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

pub const PK_FIELD: &str = "PK";
pub const SK_FIELD: &str = "SK";

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
}

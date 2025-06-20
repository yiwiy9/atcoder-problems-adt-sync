use crate::error::DdbError;
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
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

/// Represents input data for writing a contest record to DynamoDB.
#[derive(Debug, Clone)]
pub struct ContestWriteInput {
    pub start_epoch_second: u64,
    pub contest_id: String,
    pub last_fetched_submission_id: Option<u64>,
}

impl ContestWriteInput {
    /// Converts this input into a DynamoDB PutRequest.
    pub fn into_put_request(self) -> Result<PutRequest, DdbError> {
        let pk = AdtContestRecord::pk_attr()
            .as_s()
            .map_err(|_| DdbError::SerdeConversionError("PK is not a string".into()))?
            .to_owned();

        let sk = AdtContestRecord::sk_attr(self.start_epoch_second, &self.contest_id)
            .as_s()
            .map_err(|_| DdbError::SerdeConversionError("SK is not a string".into()))?
            .to_owned();

        let record = AdtContestRecord {
            pk,
            sk,
            contest_id: self.contest_id,
            last_fetched_submission_id: self.last_fetched_submission_id,
        };

        let item = serde_dynamo::to_item(&record)
            .map_err(|e| DdbError::SerdeConversionError(e.to_string()))?;

        PutRequest::builder()
            .set_item(Some(item))
            .build()
            .map_err(|e| DdbError::SerdeConversionError(e.to_string()))
    }

    /// Converts this input into a WriteRequest to be used in BatchWriteItem.
    pub fn into_write_request(self) -> Result<WriteRequest, DdbError> {
        let put_request = self.into_put_request()?;
        Ok(WriteRequest::builder().put_request(put_request).build())
    }
}

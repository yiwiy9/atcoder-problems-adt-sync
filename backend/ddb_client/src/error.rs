use aws_sdk_dynamodb::Error as AwsSdkError;
use thiserror::Error;

/// Errors returned by DynamoDB client operations.
#[derive(Debug, Error)]
pub enum DdbError {
    /// AWS SDK error during DynamoDB operation.
    #[error("AWS SDK error")]
    AwsSdkError(#[source] AwsSdkError),

    /// No item found in the table.
    #[error("Item not found")]
    NotFound,

    /// Failed to deserialize DynamoDB item into expected struct.
    #[error("Failed to deserialize item: {0}")]
    DeserializationError(String),
}

type GetItemError =
    aws_sdk_dynamodb::error::SdkError<aws_sdk_dynamodb::operation::get_item::GetItemError>;
impl From<GetItemError> for DdbError {
    fn from(source: GetItemError) -> Self {
        DdbError::AwsSdkError(source.into())
    }
}

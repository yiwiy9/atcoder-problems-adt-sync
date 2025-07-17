use aws_sdk_dynamodb::Error as AwsSdkError;
use thiserror::Error;

/// Errors returned by DynamoDB client operations.
#[derive(Debug, Error)]
pub enum DdbError {
    /// AWS SDK error during DynamoDB operation.
    #[error("AWS SDK error: {0}")]
    AwsSdkError(#[source] Box<AwsSdkError>),

    /// No item found in the table.
    #[error("Item not found")]
    NotFound,

    /// Failed to convert to or from DynamoDB item format.
    #[error("Failed to convert item: {0}")]
    SerdeConversionError(String),

    /// Failed to build a DynamoDB request.
    #[error("Failed to build DynamoDB request: {0}")]
    AwsBuildError(String),

    /// Unprocessed items exceeded retry limit.
    #[error("Unprocessed items exceeded retry limit")]
    UnprocessedItemsExceeded,
}

// === AWS SDK error conversions ===
type GetItemError =
    aws_sdk_dynamodb::error::SdkError<aws_sdk_dynamodb::operation::get_item::GetItemError>;
impl From<GetItemError> for DdbError {
    fn from(source: GetItemError) -> Self {
        DdbError::AwsSdkError(Box::new(source.into()))
    }
}

type QueryError = aws_sdk_dynamodb::error::SdkError<aws_sdk_dynamodb::operation::query::QueryError>;
impl From<QueryError> for DdbError {
    fn from(source: QueryError) -> Self {
        DdbError::AwsSdkError(Box::new(source.into()))
    }
}

type BatchGetItemError = aws_sdk_dynamodb::error::SdkError<
    aws_sdk_dynamodb::operation::batch_get_item::BatchGetItemError,
>;
impl From<BatchGetItemError> for DdbError {
    fn from(source: BatchGetItemError) -> Self {
        DdbError::AwsSdkError(Box::new(source.into()))
    }
}

type BatchWriteItemError = aws_sdk_dynamodb::error::SdkError<
    aws_sdk_dynamodb::operation::batch_write_item::BatchWriteItemError,
>;
impl From<BatchWriteItemError> for DdbError {
    fn from(source: BatchWriteItemError) -> Self {
        DdbError::AwsSdkError(Box::new(source.into()))
    }
}

// === External (non-SDK) error conversions ===
impl From<serde_dynamo::Error> for DdbError {
    fn from(err: serde_dynamo::Error) -> Self {
        DdbError::SerdeConversionError(err.to_string())
    }
}

impl From<aws_sdk_dynamodb::error::BuildError> for DdbError {
    fn from(err: aws_sdk_dynamodb::error::BuildError) -> Self {
        DdbError::AwsBuildError(err.to_string())
    }
}

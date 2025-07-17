use crate::error::DdbError;
use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};
use serde::Serialize;

/// Trait for converting models into DynamoDB write requests.
pub trait ToWriteRequest: Serialize {
    /// Converts the model into a DynamoDB `PutRequest`.
    fn to_put_request(&self) -> Result<PutRequest, DdbError> {
        let item = serde_dynamo::to_item(self)?;
        let put_request = PutRequest::builder().set_item(Some(item)).build()?;
        Ok(put_request)
    }

    /// Converts the model into a DynamoDB `WriteRequest` for BatchWriteItem.
    fn to_write_request(&self) -> Result<WriteRequest, DdbError> {
        let put_request = self.to_put_request()?;
        Ok(WriteRequest::builder().put_request(put_request).build())
    }
}

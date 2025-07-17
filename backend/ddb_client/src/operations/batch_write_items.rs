use crate::constants::{BASE_BACKOFF_MILLIS, MAX_BATCH_WRITE, MAX_RETRIES};
use crate::error::DdbError;
use crate::models::traits::ToWriteRequest;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

/// Write multiple items to DynamoDB using BatchWriteItem.
/// Items must implement the `ToWriteRequest` trait.
/// Writes in batches of 25 with retries using exponential backoff.
pub async fn batch_write_items<T: ToWriteRequest>(
    client: &Client,
    table_name: &str,
    items: Vec<T>,
) -> Result<(), DdbError> {
    let mut start = 0;

    while start < items.len() {
        let end = (start + MAX_BATCH_WRITE).min(items.len());
        let batch = &items[start..end];

        let mut write_requests = Vec::with_capacity(batch.len());
        for item in batch {
            let write_req = item.to_write_request()?;
            write_requests.push(write_req);
        }

        let mut request_items = HashMap::new();
        request_items.insert(table_name.to_string(), write_requests);

        let mut retries = 0;
        let mut backoff = BASE_BACKOFF_MILLIS;

        loop {
            let response = client
                .batch_write_item()
                .set_request_items(Some(request_items.clone()))
                .send()
                .await?;

            // Break if all items were processed successfully
            if response.unprocessed_items().is_none_or(|m| m.is_empty()) {
                break;
            }

            // Retry if unprocessed items remain
            if retries >= MAX_RETRIES {
                return Err(DdbError::UnprocessedItemsExceeded);
            }

            request_items = response.unprocessed_items().unwrap().clone();
            retries += 1;

            // Apply exponential backoff before retrying
            sleep(Duration::from_millis(backoff)).await;
            backoff *= 2;
        }

        start = end;
    }

    Ok(())
}

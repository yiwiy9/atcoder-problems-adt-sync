use crate::error::DdbError;
use crate::models::ContestWriteInput;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

use crate::constants::{BASE_BACKOFF_MILLIS, MAX_BATCH_WRITE, MAX_RETRIES};

/// Write multiple contest records to DynamoDB using BatchWriteItem.
/// Items are written in batches of 25, with retries using exponential backoff on failure.
pub async fn batch_write_contests(
    client: &Client,
    table_name: &str,
    items: Vec<ContestWriteInput>,
) -> Result<(), DdbError> {
    let mut start = 0;

    while start < items.len() {
        let end = (start + MAX_BATCH_WRITE).min(items.len());
        let batch = &items[start..end];

        let mut write_requests = Vec::with_capacity(batch.len());
        for input in batch {
            let write_req = input.clone().into_write_request()?;
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

            if response.unprocessed_items().is_none_or(|m| m.is_empty()) {
                break;
            }

            // If unprocessed items remain, retry them
            if retries >= MAX_RETRIES {
                return Err(DdbError::UnprocessedItemsExceeded);
            }

            request_items = response.unprocessed_items().unwrap().clone();
            retries += 1;
            sleep(Duration::from_millis(backoff)).await;
            backoff *= 2; // exponential backoff
        }

        start = end;
    }

    Ok(())
}

use crate::constants::{BASE_BACKOFF_MILLIS, MAX_BATCH_GET, MAX_RETRIES};
use crate::error::DdbError;
use crate::models::{
    UserAcProblemRecord,
    constants::{PK_FIELD, SK_FIELD},
};
use aws_sdk_dynamodb::{Client, types::KeysAndAttributes};
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

/// Retrieve multiple users' AC problems using BatchGetItem.
/// Batches up to 100 items per request, with retries using exponential backoff.
pub async fn batch_get_user_ac_problems(
    client: &Client,
    table_name: &str,
    user_ids: Vec<String>,
) -> Result<HashMap<String, UserAcProblemRecord>, DdbError> {
    let mut result_map = HashMap::new();
    let mut start = 0;

    while start < user_ids.len() {
        let end = (start + MAX_BATCH_GET).min(user_ids.len());
        let batch = &user_ids[start..end];

        let keys = batch
            .iter()
            .map(|user_id| {
                HashMap::from([
                    (PK_FIELD.to_string(), UserAcProblemRecord::pk_attr(user_id)),
                    (SK_FIELD.to_string(), UserAcProblemRecord::sk_attr()),
                ])
            })
            .collect::<Vec<_>>();

        let mut request_items = HashMap::new();
        request_items.insert(
            table_name.to_string(),
            KeysAndAttributes::builder().set_keys(Some(keys)).build()?,
        );

        let mut retries = 0;
        let mut backoff = BASE_BACKOFF_MILLIS;

        loop {
            let response = client
                .batch_get_item()
                .set_request_items(Some(request_items.clone()))
                .send()
                .await?;

            // Extract items from the response
            if let Some(responses) = response.responses() {
                if let Some(items) = responses.get(table_name) {
                    for item in items.clone() {
                        let record: UserAcProblemRecord = serde_dynamo::from_item(item)?;
                        let user_id = record.user_id();
                        result_map.insert(user_id, record);
                    }
                }
            }

            // Break if all keys were processed successfully
            if response.unprocessed_keys().is_none_or(|m| m.is_empty()) {
                break;
            }

            // Retry if unprocessed keys remain
            if retries >= MAX_RETRIES {
                return Err(DdbError::UnprocessedItemsExceeded);
            }

            request_items = response.unprocessed_keys().unwrap().clone();
            retries += 1;

            // Apply exponential backoff before retrying
            sleep(Duration::from_millis(backoff)).await;
            backoff *= 2;
        }

        start = end;
    }

    Ok(result_map)
}

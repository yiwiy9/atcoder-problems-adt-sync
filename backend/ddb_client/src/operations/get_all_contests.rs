use crate::error::DdbError;
use crate::models::{AdtContestRecord, constants::PK_FIELD};
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use std::collections::HashMap;

/// Retrieve all AtCoder contests from DynamoDB.
/// Optionally limit the number of items read to avoid reading too much data.
pub async fn get_all_contests(
    client: &Client,
    table_name: &str,
    max_items: Option<usize>,
) -> Result<Vec<AdtContestRecord>, DdbError> {
    let mut contests = Vec::new();
    let mut last_evaluated_key: Option<HashMap<String, AttributeValue>> = None;

    loop {
        let mut req = client
            .query()
            .table_name(table_name)
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", PK_FIELD)
            .expression_attribute_values(":pk", AdtContestRecord::pk_attr())
            .scan_index_forward(false);

        if let Some(ref lek) = last_evaluated_key {
            req = req.set_exclusive_start_key(Some(lek.clone()));
        }

        let result = req.send().await?;

        if let Some(items) = result.items {
            for item in items {
                let record = serde_dynamo::from_item(item)?;
                contests.push(record);

                if let Some(max) = max_items {
                    if contests.len() >= max {
                        return Ok(contests);
                    }
                }
            }
        }

        if let Some(lek) = result.last_evaluated_key {
            last_evaluated_key = Some(lek);
        } else {
            break;
        }
    }

    Ok(contests)
}

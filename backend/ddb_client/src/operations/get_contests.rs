use crate::error::DdbError;
use crate::models::{AdtContestRecord, constants::PK_FIELD};
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use std::collections::HashMap;

/// Retrieve ADT contests from DynamoDB.
/// Queries all partitions from current month down to ADT_START_YEAR_MONTH.
/// Optionally limit the number of items read to avoid reading too much data.
pub async fn get_contests(
    client: &Client,
    table_name: &str,
    max_items: Option<usize>,
) -> Result<Vec<AdtContestRecord>, DdbError> {
    let mut all_contests = Vec::new();
    let pks = AdtContestRecord::generate_pks_descending();

    for pk in &pks {
        let remaining_items = max_items.map(|max| max - all_contests.len());
        let partition_contests =
            query_single_partition(client, table_name, pk, remaining_items).await?;

        for contest in partition_contests {
            all_contests.push(contest);

            if let Some(max) = max_items {
                if all_contests.len() >= max {
                    return Ok(all_contests);
                }
            }
        }
    }

    Ok(all_contests)
}

/// Query a single partition (year-month) for ADT contests.
async fn query_single_partition(
    client: &Client,
    table_name: &str,
    pk: &AttributeValue,
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
            .expression_attribute_values(":pk", pk.clone())
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

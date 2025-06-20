use crate::error::DdbError;
use crate::models::{AdtContestRecord, PK_FIELD};
use aws_sdk_dynamodb::Client;

/// Retrieve the most recent contest from DynamoDB
/// by sorting by SK (start_epoch_second) in descending order (limit 1).
pub async fn get_latest_contest(
    client: &Client,
    table_name: &str,
) -> Result<AdtContestRecord, DdbError> {
    let result = client
        .query()
        .table_name(table_name)
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", PK_FIELD)
        .expression_attribute_values(":pk", AdtContestRecord::pk_attr())
        .scan_index_forward(false)
        .limit(1)
        .send()
        .await?;

    let items = result.items.ok_or(DdbError::NotFound)?;
    if items.is_empty() {
        return Err(DdbError::NotFound);
    }

    let item = items[0].clone();
    let record =
        serde_dynamo::from_item(item).map_err(|e| DdbError::SerdeConversionError(e.to_string()))?;

    Ok(record)
}

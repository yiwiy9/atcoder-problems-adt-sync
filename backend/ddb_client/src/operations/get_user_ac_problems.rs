use crate::error::DdbError;
use crate::models::{
    UserAcProblemRecord,
    constants::{PK_FIELD, SK_FIELD},
};
use aws_sdk_dynamodb::Client;

/// Retrieve the AC problems for a user from DynamoDB.
pub async fn get_user_ac_problems(
    client: &Client,
    table_name: &str,
    user_id: &str,
) -> Result<UserAcProblemRecord, DdbError> {
    let result = client
        .get_item()
        .table_name(table_name)
        .key(PK_FIELD, UserAcProblemRecord::pk_attr(user_id))
        .key(SK_FIELD, UserAcProblemRecord::sk_attr())
        .send()
        .await?;

    let item = result.item.ok_or(DdbError::NotFound)?;

    let record = serde_dynamo::from_item(item)?;

    Ok(record)
}

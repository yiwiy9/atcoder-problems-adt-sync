use crate::constants::{ATCODER_SESSION_ENV, DYNAMODB_TABLE_ENV};
use atcoder_client::AtCoderClient;
use ddb_client::DdbService;
use std::env;

/// Initializes the AtCoder client from the environment variable.
pub async fn init_atcoder_client() -> Result<AtCoderClient, String> {
    let session = env::var(ATCODER_SESSION_ENV)
        .map_err(|_| format!("Environment variable {} is not set", ATCODER_SESSION_ENV))?;

    AtCoderClient::from_revel_session(&session)
        .await
        .map_err(|e| format!("Failed to create AtCoder client: {:?}", e))
}

/// Initializes the DynamoDB service from the environment variable.
pub async fn init_ddb_service() -> Result<DdbService, String> {
    let table_name = env::var(DYNAMODB_TABLE_ENV)
        .map_err(|_| format!("Environment variable {} is not set", DYNAMODB_TABLE_ENV))?;

    Ok(DdbService::from_env(table_name).await)
}

use crate::error::DdbError;
use crate::models::{AdtContestRecord, UserAcProblemRecord, traits::ToWriteRequest};
use crate::operations;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

/// Service for interacting with DynamoDB for AtCoder Problems ADT Sync.
#[derive(Clone)]
pub struct DdbService {
    client: Client,
    table_name: String,
}

impl DdbService {
    /// Create a new DdbService by loading AWS SDK config from the environment.
    pub async fn from_env(table_name: impl Into<String>) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self {
            client,
            table_name: table_name.into(),
        }
    }

    /// Retrieve a user's AC problems from DynamoDB.
    pub async fn get_user_ac_problems(
        &self,
        user_id: &str,
    ) -> Result<UserAcProblemRecord, DdbError> {
        operations::get_user_ac_problems(&self.client, &self.table_name, user_id).await
    }

    /// Retrieve multiple users' AC problems using BatchGetItem.
    pub async fn batch_get_user_ac_problems(
        &self,
        user_ids: Vec<String>,
    ) -> Result<HashMap<String, UserAcProblemRecord>, DdbError> {
        operations::batch_get_user_ac_problems(&self.client, &self.table_name, user_ids).await
    }

    /// Retrieve the most recent contest from DynamoDB.
    pub async fn get_latest_contest(&self) -> Result<AdtContestRecord, DdbError> {
        operations::get_latest_contest(&self.client, &self.table_name).await
    }

    /// Retrieve all ADT contests (optionally limited by max count).
    pub async fn get_all_contests(
        &self,
        max_items: Option<usize>,
    ) -> Result<Vec<AdtContestRecord>, DdbError> {
        operations::get_all_contests(&self.client, &self.table_name, max_items).await
    }

    /// Write multiple items to DynamoDB using BatchWriteItem.
    pub async fn batch_write_items<T: ToWriteRequest>(
        &self,
        items: Vec<T>,
    ) -> Result<(), DdbError> {
        operations::batch_write_items(&self.client, &self.table_name, items).await
    }
}

use crate::error::DdbError;
use crate::models::{AdtContestRecord, ContestWriteInput, UserAcProblemRecord};
use crate::operations;
use aws_sdk_dynamodb::Client;

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

    /// Retrieve the most recent contest from DynamoDB.
    pub async fn get_latest_contest(&self) -> Result<AdtContestRecord, DdbError> {
        operations::get_latest_contest(&self.client, &self.table_name).await
    }

    /// Write multiple contest records to DynamoDB.
    pub async fn batch_write_contests(
        &self,
        inputs: Vec<ContestWriteInput>,
    ) -> Result<(), DdbError> {
        operations::batch_write_contests(&self.client, &self.table_name, inputs).await
    }
}

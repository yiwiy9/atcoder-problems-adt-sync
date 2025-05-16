use crate::error::DdbError;
use crate::models::UserAcProblemRecord;
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
    pub async fn new(table_name: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client, table_name }
    }

    /// Retrieve a user's AC problems from DynamoDB.
    pub async fn get_user_ac_problems(
        &self,
        user_id: &str,
    ) -> Result<UserAcProblemRecord, DdbError> {
        operations::get_user_ac_problems(&self.client, &self.table_name, user_id).await
    }
}

use crate::dto::UserAcProblemDto;
use atcoder_client::Submission;
use ddb_client::{DdbError, DdbService};
use std::collections::HashMap;

/// Updates user AC problem records in DynamoDB based on new submissions.
pub async fn sync_user_ac_problems_from_submissions(
    ddb_service: &DdbService,
    submissions: Vec<Submission>,
) -> Result<(), DdbError> {
    if submissions.is_empty() {
        return Ok(());
    }

    // Group new AC problems by user
    let mut new_map = HashMap::new();
    for dto in UserAcProblemDto::from_new_ac_submissions(submissions) {
        let record = dto.into_record();
        new_map.insert(record.user_id(), record);
    }

    let user_ids: Vec<String> = new_map.keys().cloned().collect();

    // Load existing records from DynamoDB
    let existing_map = ddb_service.batch_get_user_ac_problems(user_ids).await?;

    // Merge new and existing problems
    let mut merged = Vec::with_capacity(new_map.len());
    for (user_id, mut new_record) in new_map {
        if let Some(existing) = existing_map.get(&user_id) {
            new_record.merge_ac_problems_from(existing);
        }
        merged.push(new_record);
    }

    // Write updated records
    ddb_service.batch_write_items(merged).await?;

    Ok(())
}

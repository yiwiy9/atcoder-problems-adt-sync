use crate::content::chrome_message;
use crate::content::database::{SubmissionDb, UserDb};
use crate::log;
use crate::models::User;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

/// Sync ADT submissions for the given user and clean up submissions for all other users.
pub async fn sync_and_cleanup(user_id: &str) -> Result<(), JsValue> {
    futures::try_join!(sync(user_id), cleanup(vec![user_id]))?;
    Ok(())
}

/// Sync ADT submissions for the given user.
async fn sync(user_id: &str) -> Result<(), JsValue> {
    let user = User::new(user_id);

    let problems = chrome_message::get_problems_by_user(&user).await?;
    if problems.is_empty() {
        log::info(format!("No submissions found for user {}", &user.id));
        return Ok(());
    }

    let user_db = UserDb::new().await?;
    user_db.add(&user).await?;

    let submission_db = SubmissionDb::new(&user.id).await?;

    for problem in problems {
        submission_db.add_problem(&problem).await?;
    }

    log::info(format!("Synced submissions for user {}", user.id));

    Ok(())
}

/// Clean up ADT submissions for all users except those in `exclude_user_ids`.
pub async fn cleanup(exclude_user_ids: Vec<&str>) -> Result<(), JsValue> {
    let exclude_set = exclude_user_ids.into_iter().collect::<HashSet<_>>();
    let user_db = UserDb::new().await?;

    let target_users = user_db
        .get_all()
        .await?
        .into_iter()
        .filter(|user| !exclude_set.contains(user.id.as_str()))
        .collect::<Vec<_>>();

    for user in target_users {
        let submission_db = SubmissionDb::new(&user.id).await?;

        match submission_db.cleanup().await {
            Ok(count) => log::info(format!(
                "Cleaned up {} submissions for user {}",
                count, user.id
            )),
            Err(e) => {
                log::error(format!(
                    "Failed to clean up submissions for user {}: {:?}",
                    user.id, e
                ));
                continue;
            }
        }

        if let Err(e) = user_db.remove(&user).await {
            log::error(format!("Failed to remove user {}: {:?}", user.id, e));
            continue;
        }

        log::info(format!("Removed user {}", user.id));
    }

    Ok(())
}

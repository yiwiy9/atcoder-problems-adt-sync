use atcoder_problems_adt_sync_batch::{
    client::{init_atcoder_client, init_ddb_service},
    constants::{ATCODER_CRAWL_SLEEP_MILLIS, MAX_IN_MEMORY_SUBMISSIONS},
    crawler::SubmissionCrawler,
    dto::AdtContestDto,
    service::sync_user_ac_problems_from_submissions,
};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Initialize AtCoder client
    let atcoder_client = match init_atcoder_client().await {
        Ok(client) => {
            log::info!("Successfully created AtCoder client");
            client
        }
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Initialize DynamoDB service
    let ddb_service = match init_ddb_service().await {
        Ok(service) => {
            log::info!("Successfully created DynamoDB service");
            service
        }
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Fetch all contests from DynamoDB
    let contest_records = match ddb_service.get_all_contests(None).await {
        Ok(records) => {
            log::info!(
                "Successfully fetched all contests from DynamoDB: {} records",
                records.len()
            );
            records
        }
        Err(err) => {
            log::error!("Failed to fetch all contests from DynamoDB: {}", err);
            return;
        }
    };

    // Initialize SubmissionCrawler with AtCoder client
    let submission_crawler = SubmissionCrawler::new(atcoder_client);

    let mut new_ac_submissions = vec![];
    let mut update_contests = vec![];

    // Crawl submissions for each contest
    for record in contest_records {
        // Sleep before processing each contest to maintain consistent intervals
        sleep(Duration::from_millis(ATCODER_CRAWL_SLEEP_MILLIS)).await;

        log::info!("Crawling submissions for contest: {}", record.contest_id);

        // Crawl submissions until the last fetched submission ID
        let contest_ac_submissions = match submission_crawler
            .crawl(&record.contest_id, record.last_fetched_submission_id)
            .await
        {
            Ok(submissions) => submissions
                .into_iter()
                .filter(|s| s.is_accepted())
                .collect::<Vec<_>>(),
            Err(_) => continue,
        };

        if contest_ac_submissions.is_empty() {
            log::info!("No new AC submissions for contest: {}", record.contest_id);
            continue;
        }

        update_contests.push(
            AdtContestDto {
                start_epoch_second: record.start_epoch_second(),
                contest_id: record.contest_id.clone(),
                last_fetched_submission_id: contest_ac_submissions.first().map(|s| s.id),
            }
            .into_record(),
        );

        new_ac_submissions.extend(contest_ac_submissions);

        if new_ac_submissions.len() >= MAX_IN_MEMORY_SUBMISSIONS {
            log::warn!(
                "Reached maximum in-memory submissions limit: {}. Writing to DynamoDB.",
                MAX_IN_MEMORY_SUBMISSIONS
            );

            log::info!(
                "Writing {} AC submissions to DynamoDB.",
                new_ac_submissions.len()
            );

            // Write the new AC submissions to DynamoDB
            if let Err(e) =
                sync_user_ac_problems_from_submissions(&ddb_service, new_ac_submissions).await
            {
                log::error!("Failed to write submissions to DynamoDB: {}", e);
            } else {
                log::info!("Successfully wrote submissions to DynamoDB.");
            }

            // Clear the in-memory submissions
            new_ac_submissions = vec![];
        }
    }

    // Flush remaining AC submissions to DynamoDB
    if !new_ac_submissions.is_empty() {
        log::info!(
            "Writing remaining {} AC submissions to DynamoDB.",
            new_ac_submissions.len()
        );

        if let Err(e) =
            sync_user_ac_problems_from_submissions(&ddb_service, new_ac_submissions).await
        {
            log::error!("Failed to write remaining submissions to DynamoDB: {}", e);
        } else {
            log::info!("Successfully wrote remaining submissions to DynamoDB.");
        }
    }

    // Write updated contest records to DynamoDB
    if !update_contests.is_empty() {
        log::info!(
            "Updating {} contest records in DynamoDB.",
            update_contests.len()
        );

        if let Err(e) = ddb_service.batch_write_items(update_contests).await {
            log::error!("Failed to update contest records in DynamoDB: {}", e);
        } else {
            log::info!("Successfully updated contest records in DynamoDB.");
        }
    }

    log::info!("Crawling and writing submissions completed successfully");
}

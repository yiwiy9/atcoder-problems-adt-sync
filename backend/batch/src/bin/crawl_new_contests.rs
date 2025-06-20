use atcoder_problems_adt_sync_batch::{
    client::{init_atcoder_client, init_ddb_service},
    crawler::ContestCrawler,
};
use ddb_client::{ContestWriteInput, DdbError};

/// Main function to crawl AtCoder contests and write them to DynamoDB.
/// Skips already stored contests using the latest contest ID.
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

    // Fetch the latest contest ID from DynamoDB
    let last_fetched_contest_id = match ddb_service.get_latest_contest().await {
        Ok(record) => {
            log::info!("Successfully fetched latest contest: {}", record.contest_id);
            Some(record.contest_id)
        }
        Err(DdbError::NotFound) => {
            log::warn!("No contests found in DynamoDB, starting from scratch");
            None
        }
        Err(err) => {
            log::error!("Failed to fetch latest contest from DynamoDB: {}", err);
            return;
        }
    };

    // Initialize ContestCrawler with AtCoder client
    let contest_crawler = ContestCrawler::new(atcoder_client);

    // Crawl contests until the last fetched contest ID
    let mut contests = match contest_crawler
        .crawl(last_fetched_contest_id.as_deref())
        .await
    {
        Ok(contests) => contests,
        Err(_) => return,
    };
    contests.sort_by_key(|c| c.start_epoch_second);

    // Convert contests to ContestWriteInput format
    let contest_write_inputs = contests
        .into_iter()
        .map(|c| ContestWriteInput {
            start_epoch_second: c.start_epoch_second,
            contest_id: c.id,
            last_fetched_submission_id: None, // Assuming we don't have this info yet
        })
        .collect::<Vec<_>>();

    log::info!("Total contests to write: {}", contest_write_inputs.len());

    // Write contests to DynamoDB
    if let Err(err) = ddb_service.batch_write_contests(contest_write_inputs).await {
        log::error!("Failed to write contests to DynamoDB: {}", err);
        return;
    }

    log::info!("Crawling and writing contests completed successfully");
}

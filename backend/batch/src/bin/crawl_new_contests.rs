use atcoder_problems_adt_sync_batch::{
    client::{init_atcoder_client, init_ddb_service},
    crawler::ContestCrawler,
    dto::AdtContestDto,
};
use ddb_client::DdbError;

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

    let contest_write_records = AdtContestDto::from_new_contests(contests)
        .into_iter()
        .map(|dto| dto.into_record())
        .collect::<Vec<_>>();

    log::info!(
        "Total contests to write: {} records",
        contest_write_records.len()
    );

    // Write contests to DynamoDB
    if let Err(err) = ddb_service.batch_write_items(contest_write_records).await {
        log::error!("Failed to write contests to DynamoDB: {}", err);
        return;
    }

    log::info!("Crawling and writing contests completed successfully");
}

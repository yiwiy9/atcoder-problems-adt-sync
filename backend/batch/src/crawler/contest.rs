use crate::constants::ATCODER_CRAWL_SLEEP_MILLIS;
use atcoder_client::{AtCoderClient, AtCoderClientError, Contest};
use tokio::time::{Duration, sleep};

/// Provides functionality to crawl the AtCoder contest archive page by page.
pub struct ContestCrawler {
    client: AtCoderClient,
}

impl ContestCrawler {
    pub fn new(client: AtCoderClient) -> Self {
        Self { client }
    }

    /// Crawl contest list page by page until `until_contest_id` is found (exclusive).
    /// If `None`, continue until the end of the archive (determined by an empty or missing page).
    pub async fn crawl(
        &self,
        until_contest_id: Option<&str>,
    ) -> Result<Vec<Contest>, AtCoderClientError> {
        log::info!("Starting contest crawl");
        if let Some(id) = until_contest_id {
            log::info!("Crawling until contest ID: {}", id);
        } else {
            log::info!("Crawling all contests until the end of archive");
        }

        let mut all_contests = Vec::new();
        let mut page = 1;

        'outer: loop {
            log::info!("Fetching contest archive page {}", page);
            let contests = match self.client.fetch_adt_contests(page).await {
                Ok(c) => c,
                Err(e) => {
                    if e.is_empty_content() {
                        log::info!("Reached end of archive at page {}", page);
                        break;
                    } else {
                        log::error!("Failed to fetch page {}: {:?}", page, e);
                        return Err(e);
                    }
                }
            };

            if contests.is_empty() {
                log::warn!("No contests found on page {}, stopping.", page);
                break;
            }

            log::info!("Fetched {} contests from page {}", contests.len(), page);

            for contest in contests {
                if let Some(stop_id) = until_contest_id {
                    if contest.id == stop_id {
                        log::info!("Reached contest ID {}, stopping.", stop_id);
                        break 'outer;
                    }
                }
                all_contests.push(contest);
            }

            page += 1;
            sleep(Duration::from_millis(ATCODER_CRAWL_SLEEP_MILLIS)).await;
        }

        log::info!(
            "Crawling completed, total contests fetched: {}",
            all_contests.len()
        );

        Ok(all_contests)
    }
}

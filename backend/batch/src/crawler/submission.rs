use crate::constants::{
    ATCODER_CRAWL_MAX_RETRIES, ATCODER_CRAWL_RETRY_SLEEP_MILLIS, ATCODER_CRAWL_SLEEP_MILLIS,
};
use atcoder_client::{AtCoderClient, AtCoderClientError, Submission};
use tokio::time::{Duration, sleep};

/// Provides functionality to crawl the AtCoder submission page by page.
pub struct SubmissionCrawler {
    client: AtCoderClient,
}

impl SubmissionCrawler {
    pub fn new(client: AtCoderClient) -> Self {
        Self { client }
    }

    /// Fetch submissions for a given contest page with retry logic.
    async fn fetch_submissions_with_retry(
        &self,
        contest_id: &str,
        page: u32,
    ) -> Result<Vec<Submission>, AtCoderClientError> {
        for attempt in 1..=ATCODER_CRAWL_MAX_RETRIES {
            match self.client.fetch_submissions(contest_id, page).await {
                Ok(submissions) => return Ok(submissions),
                Err(e) if e.is_retryable() => {
                    log::warn!(
                        "Retrying request for contest {} on page {} (attempt {}/{}) after {}ms: {}",
                        contest_id,
                        page,
                        attempt,
                        ATCODER_CRAWL_MAX_RETRIES,
                        ATCODER_CRAWL_RETRY_SLEEP_MILLIS,
                        e
                    );
                    sleep(Duration::from_millis(ATCODER_CRAWL_RETRY_SLEEP_MILLIS)).await;
                }
                Err(e) => return Err(e),
            }
        }

        self.client.fetch_submissions(contest_id, page).await
    }

    /// Crawl submission list for a given contest page by page until `until_submission_id` is found (exclusive).
    /// If `None`, continue until the end of submissions (determined by an empty or missing page).
    pub async fn crawl(
        &self,
        contest_id: &str,
        until_submission_id: Option<u64>,
    ) -> Result<Vec<Submission>, AtCoderClientError> {
        log::debug!("Starting submission crawl");
        if let Some(id) = until_submission_id {
            log::debug!("Crawling until submission ID: {}", id);
        } else {
            log::debug!("Crawling all submissions until the end of submissions");
        }

        let mut all_submissions = Vec::new();
        let mut page = 1;

        'outer: loop {
            log::debug!("Fetching submission page {}", page);
            let submissions = match self.fetch_submissions_with_retry(contest_id, page).await {
                Ok(s) => s,
                Err(e) => {
                    if e.is_empty_content() {
                        log::info!(
                            "Reached end of submissions for contest {} on page {}",
                            contest_id,
                            page
                        );
                        break;
                    } else {
                        log::error!(
                            "Failed to fetch submissions for contest {} on page {}: {}",
                            contest_id,
                            page,
                            e
                        );
                        return Err(e);
                    }
                }
            };

            if submissions.is_empty() {
                log::warn!("No submissions found on page {}, stopping.", page);
                break;
            }

            log::debug!(
                "Fetched {} submissions for contest {} on page {}",
                submissions.len(),
                contest_id,
                page
            );

            for submission in submissions {
                if let Some(stop_id) = until_submission_id {
                    if submission.id == stop_id {
                        log::debug!("Reached submission ID {}, stopping.", stop_id);
                        break 'outer;
                    }
                }
                all_submissions.push(submission);
            }

            page += 1;
            sleep(Duration::from_millis(ATCODER_CRAWL_SLEEP_MILLIS)).await;
        }

        log::debug!(
            "Crawling completed, total submissions for contest {} fetched: {}",
            contest_id,
            all_submissions.len()
        );

        Ok(all_submissions)
    }
}

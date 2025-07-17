use crate::constants::{ATCODER_BASE_URL, TEST_CONTEST_ID};
use crate::error::AtCoderClientError;
use crate::models::{Contest, Submission};
use crate::scraper::{scrape_contest_page, scrape_submission_page};
use reqwest::{Client, StatusCode, Url, cookie::Jar, redirect::Policy};
use std::{str::FromStr, sync::Arc};

/// A client for scraping AtCoder pages using an authenticated REVEL_SESSION.
#[derive(Clone)]
pub struct AtCoderClient {
    client: Client,
}

impl AtCoderClient {
    /// Creates a new client using the given REVEL_SESSION cookie.
    /// Verifies the session by accessing a known submissions page.
    pub async fn from_revel_session(session: &str) -> Result<Self, AtCoderClientError> {
        let cookie_header = format!("REVEL_SESSION={}", session);
        let url = Url::from_str(ATCODER_BASE_URL).expect("Hardcoded base URL should be valid");

        let jar = Jar::default();
        jar.add_cookie_str(&cookie_header, &url);

        let client = Client::builder()
            .cookie_provider(Arc::new(jar))
            .redirect(Policy::none())
            .build()?;

        let this = Self { client };

        let test_url = Self::contest_submissions_url(TEST_CONTEST_ID, 1);
        this.get_html(&test_url).await?;

        Ok(this)
    }

    /// Fetches and parses the ADT contests archive page.
    pub async fn fetch_adt_contests(&self, page: u32) -> Result<Vec<Contest>, AtCoderClientError> {
        let url = Self::adt_archive_url(page);
        let html = self.get_html(&url).await?;

        let contests = scrape_contest_page(&html)?;
        Ok(contests)
    }

    /// Fetches and parses the submissions page for a given contest.
    pub async fn fetch_submissions(
        &self,
        contest_id: &str,
        page: u32,
    ) -> Result<Vec<Submission>, AtCoderClientError> {
        let url = Self::contest_submissions_url(contest_id, page);
        let html = self.get_html(&url).await?;

        let submissions = scrape_submission_page(&html, contest_id)?;
        Ok(submissions)
    }

    /// Performs a GET request and returns the HTML as a string.
    async fn get_html(&self, url: &str) -> Result<String, AtCoderClientError> {
        let response = self
            .client
            .get(url)
            .header("accept", "text/html")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(match status {
                // Redirects (e.g., to /login) indicate an invalid or expired session
                StatusCode::FOUND | StatusCode::UNAUTHORIZED => AtCoderClientError::InvalidSession,
                StatusCode::FORBIDDEN => AtCoderClientError::Forbidden,
                StatusCode::NOT_FOUND => AtCoderClientError::NotFound,
                status if status.is_server_error() => AtCoderClientError::ServerError(status),
                _ => AtCoderClientError::UnexpectedHttpStatus(status),
            });
        }

        response
            .text()
            .await
            .map_err(AtCoderClientError::ReqwestError)
    }

    /// Constructs the URL for the ADT contest archive.
    fn adt_archive_url(page: u32) -> String {
        format!(
            "{}/contests/archive?category=60&lang=ja&page={}",
            ATCODER_BASE_URL, page
        )
    }

    /// Constructs the URL for a contest's submissions page.
    fn contest_submissions_url(contest_id: &str, page: u32) -> String {
        format!(
            "{}/contests/{}/submissions?lang=ja&page={}",
            ATCODER_BASE_URL, contest_id, page
        )
    }
}

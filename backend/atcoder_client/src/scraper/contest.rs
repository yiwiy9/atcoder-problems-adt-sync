use crate::error::AtCoderClientError;
use crate::models::Contest;
use chrono::DateTime;
use scraper::{Html, Selector};

/// Parses the HTML of the AtCoder contest archive page and returns a list of contests.
pub fn scrape(html: &str) -> Result<Vec<Contest>, AtCoderClientError> {
    let document = Html::parse_document(html);

    let tbody_selector = Selector::parse("tbody").unwrap();
    let tbody = document
        .select(&tbody_selector)
        .next()
        .ok_or(AtCoderClientError::EmptyContents)?;

    let tr_selector = Selector::parse("tr").unwrap();
    tbody
        .select(&tr_selector)
        .map(|tr| {
            let td_selector = Selector::parse("td").unwrap();
            let mut tds = tr.select(&td_selector);

            let start_text = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let start = DateTime::parse_from_str(start_text, "%Y-%m-%d %H:%M:%S%z")
                .map_err(|_| AtCoderClientError::HtmlParseError)?;
            let start = start.timestamp() as u64;

            let contest_td = tds.next().ok_or(AtCoderClientError::HtmlParseError)?;
            let a_selector = Selector::parse("a").unwrap();
            let contest_title = contest_td
                .select(&a_selector)
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let contest_link = contest_td
                .select(&a_selector)
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .value()
                .attr("href")
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let contest_id = contest_link
                .rsplit('/')
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            let duration_text = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let mut duration_split = duration_text.split(':');
            let hours = duration_split
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .parse::<u64>()
                .map_err(|_| AtCoderClientError::HtmlParseError)?;
            let minutes = duration_split
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .parse::<u64>()
                .map_err(|_| AtCoderClientError::HtmlParseError)?;
            let duration = hours * 3600 + minutes * 60;

            let rated_text = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            Ok(Contest {
                id: contest_id.to_owned(),
                start_epoch_second: start,
                duration_second: duration,
                title: contest_title.to_owned(),
                rate_change: rated_text.to_owned(),
            })
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrape_contests_with_valid_html_returns_contests() {
        let contests_page_str = include_str!("../../test_resources/contests_page.txt");
        let contests = scrape(contests_page_str).expect("contest scraping should succeed");

        assert_eq!(contests.len(), 50);

        let expected = Contest {
            id: "adt_all_20250522_3".to_string(),
            start_epoch_second: 1747913400,
            duration_second: 3600,
            title: "AtCoder Daily Training ALL 2025/05/22 20:30start".to_string(),
            rate_change: "-".to_string(),
        };

        assert_eq!(contests[0], expected);
    }

    #[test]
    fn scrape_contests_with_invalid_html_returns_empty_contents_error() {
        let contents =
            "<html><head><title>No contests</title></head><body><p>Empty</p></body></html>";
        let result = scrape(contents);

        assert!(matches!(result, Err(AtCoderClientError::EmptyContents)));
    }
}

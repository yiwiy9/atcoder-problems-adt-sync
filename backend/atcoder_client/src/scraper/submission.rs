use crate::error::AtCoderClientError;
use crate::models::Submission;
use chrono::DateTime;
use regex::Regex;
use scraper::{Html, Selector};

/// Parses the HTML of the AtCoder submission list page and returns a list of submissions.
pub fn scrape(html: &str, contest_id: &str) -> Result<Vec<Submission>, AtCoderClientError> {
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

            let time_text = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let time = DateTime::parse_from_str(time_text, "%Y-%m-%d %H:%M:%S%z")
                .map_err(|_| AtCoderClientError::HtmlParseError)?;
            let epoch_second = time.timestamp() as u64;

            let a_selector = Selector::parse("a").unwrap();

            let problem_link = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .select(&a_selector)
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .value()
                .attr("href")
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let problem_id = problem_link
                .rsplit('/')
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            let user_link = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .select(&a_selector)
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .value()
                .attr("href")
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let user_id = user_link
                .rsplit('/')
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            let language = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            let point = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .parse::<f64>()
                .map_err(|_| AtCoderClientError::HtmlParseError)?;

            let length = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .replace("Byte", "")
                .trim()
                .parse::<u64>()
                .map_err(|_| AtCoderClientError::HtmlParseError)?;

            let result = tds
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .text()
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?;

            let execution_time = tds
                .next()
                .and_then(|e| e.text().next())
                .map(|s| s.replace("ms", ""))
                .and_then(|s| s.trim().parse::<u64>().ok());

            let re = Regex::new(r"submissions/\d+$").unwrap();
            let submission_link = tr
                .select(&a_selector)
                .find(|e| match e.value().attr("href") {
                    Some(href) => re.is_match(href),
                    None => false,
                })
                .ok_or(AtCoderClientError::HtmlParseError)?
                .value()
                .attr("href")
                .ok_or(AtCoderClientError::HtmlParseError)?;
            let id = submission_link
                .rsplit('/')
                .next()
                .ok_or(AtCoderClientError::HtmlParseError)?
                .trim()
                .parse::<u64>()
                .map_err(|_| AtCoderClientError::HtmlParseError)?;

            Ok(Submission {
                id,
                epoch_second,
                problem_id: problem_id.to_owned(),
                contest_id: contest_id.to_owned(),
                user_id: user_id.to_owned(),
                language: language.to_owned(),
                point,
                length,
                result: result.to_owned(),
                execution_time,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrape_submissions_with_valid_html_returns_submissions() {
        let contest_id = "adt_all_20250522_3";
        let submissions_page_str = include_str!("../../test_resources/submissions_page.txt");
        let submissions =
            scrape(submissions_page_str, contest_id).expect("submission scraping should succeed");

        assert_eq!(submissions.len(), 20);

        let expected_0 = Submission {
            id: 66203973,
            epoch_second: 1748344794,
            problem_id: "abc369_e".to_string(),
            contest_id: contest_id.to_owned(),
            user_id: "test1".to_owned(),
            language: "C++ 20 (gcc 12.2)".to_owned(),
            point: 450.0,
            length: 3531,
            result: "AC".to_owned(),
            execution_time: Some(224),
        };
        assert_eq!(submissions[0], expected_0);

        let expected_4 = Submission {
            id: 66194430,
            epoch_second: 1748314246,
            problem_id: "abc281_a".to_string(),
            contest_id: contest_id.to_owned(),
            user_id: "test3".to_owned(),
            language: "Python (PyPy 3.10-v7.3.12)".to_owned(),
            point: 0.0,
            length: 48,
            result: "WA".to_owned(),
            execution_time: Some(62),
        };
        assert_eq!(submissions[4], expected_4);

        let expected_15 = Submission {
            id: 66184195,
            epoch_second: 1748255371,
            problem_id: "abc278_a".to_string(),
            contest_id: contest_id.to_owned(),
            user_id: "test1".to_owned(),
            language: "C++ 20 (gcc 12.2)".to_owned(),
            point: 0.0,
            length: 2470,
            result: "CE".to_owned(),
            execution_time: None,
        };
        assert_eq!(submissions[15], expected_15);

        let expected_19 = Submission {
            id: 66178451,
            epoch_second: 1748238500,
            problem_id: "abc344_c".to_string(),
            contest_id: contest_id.to_owned(),
            user_id: "test4".to_owned(),
            language: "C (gcc 12.2.0)".to_owned(),
            point: 0.0,
            length: 1276,
            result: "TLE".to_owned(),
            execution_time: Some(2207),
        };
        assert_eq!(submissions[19], expected_19);
    }

    #[test]
    fn scrape_submissions_with_invalid_html_returns_empty_contents_error() {
        let contents =
            "<html><head><title>No contests</title></head><body><p>Empty</p></body></html>";
        let result = scrape(contents, "test_contest");

        assert!(matches!(result, Err(AtCoderClientError::EmptyContents)));
    }
}

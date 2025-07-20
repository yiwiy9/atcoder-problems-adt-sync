use crate::models::traits::ToWriteRequest;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single ADT contest record stored in DynamoDB.
/// PK: "CONTEST#{YYYYMM}", SK: "{start_epoch_second}-{difficulty_order}"
#[derive(Debug, Serialize, Deserialize)]
pub struct AdtContestRecord {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub contest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetched_submission_id: Option<u64>,
}

impl AdtContestRecord {
    /// ADT start year
    const ADT_START_YEAR: u32 = 2023;
    /// ADT start month
    const ADT_START_MONTH: u32 = 10;

    /// Generate the partition key (PK) AttributeValue from year-month string.
    pub fn pk_attr(year_month: &str) -> AttributeValue {
        AttributeValue::S(format!("CONTEST#{}", year_month))
    }

    /// Generate the partition key (PK) AttributeValue from epoch seconds.
    pub fn pk_attr_from_epoch(start_epoch_second: u64) -> AttributeValue {
        let year_month = DateTime::from_timestamp(start_epoch_second as i64, 0)
            .map(|dt| dt.format("%Y%m").to_string())
            .unwrap_or_else(|| format!("{:04}{:02}", Self::ADT_START_YEAR, Self::ADT_START_MONTH));
        Self::pk_attr(&year_month)
    }

    /// Generate the sort key (SK) AttributeValue based on contest start time.
    pub fn sk_attr(start_epoch_second: u64, contest_id: &str) -> AttributeValue {
        let order = Self::difficulty_order(contest_id);
        let sk = format!("{:010}-{:02}", start_epoch_second, order);
        AttributeValue::S(sk)
    }

    /// Generate all partition keys from current month down to ADT_START_YEAR_MONTH in descending order.
    /// Returns a vector of AttributeValue representing PKs like ["CONTEST_202507", "CONTEST_202506", ..., "CONTEST_202310"]
    pub fn generate_pks_descending() -> Vec<AttributeValue> {
        let now = Utc::now();
        let current_year = now.year() as u32;
        let current_month = now.month();

        let start_year = Self::ADT_START_YEAR;
        let start_month = Self::ADT_START_MONTH;

        let mut pks = Vec::new();
        let mut year = current_year;
        let mut month = current_month;

        loop {
            let year_month = format!("{:04}{:02}", year, month);
            pks.push(Self::pk_attr(&year_month));

            if year == start_year && month == start_month {
                break;
            }

            if month == 1 {
                month = 12;
                year -= 1;
            } else {
                month -= 1;
            }

            // Safety check to prevent infinite loop
            if year < start_year || (year == start_year && month < start_month) {
                break;
            }
        }

        pks
    }

    pub fn start_epoch_second(&self) -> u64 {
        self.sk
            .split('-')
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
    }

    fn difficulty_order(contest_id: &str) -> u8 {
        if contest_id.contains("_easy") {
            1
        } else if contest_id.contains("_medium") {
            2
        } else if contest_id.contains("_hard") {
            3
        } else {
            4 // assume "_all" or default
        }
    }
}

impl ToWriteRequest for AdtContestRecord {}

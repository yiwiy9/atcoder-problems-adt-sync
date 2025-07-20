use atcoder_client::Contest;
use ddb_client::AdtContestRecord;

/// Data Transfer Object (DTO) for adt contests.
#[derive(Debug, Clone)]
pub struct AdtContestDto {
    pub start_epoch_second: u64,
    pub contest_id: String,
    pub last_fetched_submission_id: Option<u64>,
}

impl AdtContestDto {
    /// Converts this DTO into an AdtContestRecord for DynamoDB storage.
    pub fn into_record(self) -> AdtContestRecord {
        let pk = AdtContestRecord::pk_attr_from_epoch(self.start_epoch_second)
            .as_s()
            .expect("PK must be a string")
            .to_owned();

        let sk = AdtContestRecord::sk_attr(self.start_epoch_second, &self.contest_id)
            .as_s()
            .expect("SK must be a string")
            .to_owned();

        AdtContestRecord {
            pk,
            sk,
            contest_id: self.contest_id,
            last_fetched_submission_id: self.last_fetched_submission_id,
        }
    }

    /// Converts a list of crawled contests into DTOs for DynamoDB writing.
    pub fn from_new_contests<I>(new_contests: I) -> Vec<Self>
    where
        I: IntoIterator<Item = Contest>,
    {
        new_contests
            .into_iter()
            .map(|c| Self {
                start_epoch_second: c.start_epoch_second,
                contest_id: c.id,
                last_fetched_submission_id: None,
            })
            .collect()
    }
}

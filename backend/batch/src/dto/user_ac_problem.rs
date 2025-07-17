use atcoder_client::Submission;
use ddb_client::UserAcProblemRecord;
use std::collections::{BTreeMap, BTreeSet};

/// Data Transfer Object (DTO) for user accepted problems.
#[derive(Debug, Clone)]
pub struct UserAcProblemDto {
    pub user_id: String,
    pub ac_problems: BTreeSet<String>,
}

impl UserAcProblemDto {
    /// Converts this DTO into an UserAcProblemRecord for DynamoDB storage.
    pub fn into_record(self) -> UserAcProblemRecord {
        let pk = UserAcProblemRecord::pk_attr(&self.user_id)
            .as_s()
            .expect("PK must be a string")
            .to_owned();

        let sk = UserAcProblemRecord::sk_attr()
            .as_s()
            .expect("SK must be a string")
            .to_owned();

        UserAcProblemRecord {
            pk,
            sk,
            ac_problems: self.ac_problems.into_iter().collect(),
        }
    }

    /// Converts a list of crawled AC submissions into DTOs for DynamoDB writing.
    pub fn from_new_ac_submissions<I>(new_ac_submissions: I) -> Vec<Self>
    where
        I: IntoIterator<Item = Submission>,
    {
        let mut user_to_problems: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for submission in new_ac_submissions {
            user_to_problems
                .entry(submission.user_id)
                .or_default()
                .insert(submission.problem_id);
        }

        user_to_problems
            .into_iter()
            .map(|(user_id, ac_problems)| UserAcProblemDto {
                user_id,
                ac_problems,
            })
            .collect()
    }
}

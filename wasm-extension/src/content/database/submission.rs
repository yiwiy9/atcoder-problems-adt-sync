use crate::database::Database;
use crate::models::Problem;
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

pub struct SubmissionDb {
    db: Database,
    user_id: String,
}

impl SubmissionDb {
    const IDB_VERSION: u32 = 3;
    const STORE_NAME: &str = "submissions";
    const STORE_KEY: &str = "id";

    pub async fn new(user_id: &str) -> Result<Self, JsValue> {
        let db = Database::new(
            &format!("{}-{}", user_id, Self::STORE_NAME),
            Self::IDB_VERSION,
            Box::new(|db: &web_sys::IdbDatabase| {
                let options = web_sys::IdbObjectStoreParameters::new();
                options.set_key_path(&Self::STORE_KEY.into());
                let _ = db.create_object_store_with_optional_parameters(Self::STORE_NAME, &options);
            }),
        )
        .await?;
        Ok(Self {
            db,
            user_id: user_id.to_string(),
        })
    }

    pub async fn add_problem(&self, problem: &Problem) -> Result<(), JsValue> {
        let submission = Submission::new(&self.user_id, problem);
        self.db.put(Self::STORE_NAME, &submission).await
    }

    pub async fn cleanup(&self) -> Result<usize, JsValue> {
        let submissions = self.db.get_all::<Submission>(Self::STORE_NAME).await?;

        let adt_submissions = submissions
            .into_iter()
            .filter(|submission| submission.is_adt())
            .collect::<Vec<_>>();

        for adt_submission in &adt_submissions {
            self.db.delete(Self::STORE_NAME, &adt_submission.id).await?;
        }

        Ok(adt_submissions.len())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Submission {
    id: i64,
    epoch_second: i64, // The time of submission (Unix timestamp)
    problem_id: String,
    contest_id: String,
    user_id: String,
    language: String,
    point: f64,
    length: i32,
    result: String,
    execution_time: Option<i32>,

    // Additional fields for this extension
    #[serde(skip_serializing_if = "Option::is_none")]
    adt: Option<bool>,
}

impl Submission {
    /// Max safe integer for JavaScript Number (2^53 - 1)
    const JS_MAX_SAFE_INTEGER: i64 = (1 << 53) - 1;

    const LANGUAGE: &str = "ADT";
    const POINT: f64 = 0.0;
    const LENGTH: i32 = 0;
    const RESULT: &str = "AC";

    pub fn new(user_id: &str, problem: &Problem) -> Self {
        // The app compares the number of local and server data for submissions older than (latest submission time - 2 days)
        // If the latest submission time is 2 days ago, syncing ADT data to match the latest time would cause a mismatch upon a new submission.
        // Therefore, set the submission time to the current time. Ensure ADT data is stored only after the app has updated its latest submission data.
        let now = js_sys::Date::now() as i64 / 1000;

        Self {
            id: Self::generate_adt_id(&problem.id),
            epoch_second: now,
            problem_id: problem.id.to_string(),
            contest_id: problem.contest_id.to_string(),
            user_id: user_id.to_string(),
            language: Self::LANGUAGE.to_string(),
            point: Self::POINT,
            length: Self::LENGTH,
            result: Self::RESULT.to_string(),
            execution_time: None,
            adt: Some(true),
        }
    }

    pub fn is_adt(&self) -> bool {
        self.adt.unwrap_or(false)
    }

    /// Generates a unique **negative** ID for ADT submissions.
    /// - Normal submission IDs are always positive.
    /// - ADT submission IDs are **negative** (in the range `-(1 << 53) 〜 -1`).
    /// - Uses a 53-bit hash to fit within JavaScript’s Number range.
    fn generate_adt_id(problem_id: &str) -> i64 {
        let mut hasher = DefaultHasher::new();
        problem_id.hash(&mut hasher);
        let hash = hasher.finish() as i64;

        // Keep it within JavaScript's safe integer range
        let safe_hash = hash & Self::JS_MAX_SAFE_INTEGER;
        -safe_hash // Ensure the ID is always negative
    }
}

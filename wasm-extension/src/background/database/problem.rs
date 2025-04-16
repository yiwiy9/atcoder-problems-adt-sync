use crate::database::Database;
use crate::models::{Problem, User};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

pub struct ProblemCacheDb {
    db: Database,
}

impl ProblemCacheDb {
    const IDB_VERSION: u32 = 1;
    const STORE_NAME: &str = "adt_problem_cache";
    const STORE_KEY: &str = "user_id";

    pub async fn new() -> Result<Self, JsValue> {
        let db = Database::new(
            Self::STORE_NAME,
            Self::IDB_VERSION,
            Box::new(|db: &web_sys::IdbDatabase| {
                let options = web_sys::IdbObjectStoreParameters::new();
                options.set_key_path(&Self::STORE_KEY.into());
                let _ = db.create_object_store_with_optional_parameters(Self::STORE_NAME, &options);
            }),
        )
        .await?;
        Ok(Self { db })
    }

    pub async fn get(&self, user: &User) -> Result<Option<Vec<Problem>>, JsValue> {
        let maybe_cache: Option<ProblemCache> = self.db.get(Self::STORE_NAME, &user.id).await?;

        Ok(maybe_cache.filter(|cache| cache.is_fresh()).map(|cache| {
            cache
                .problem_ids
                .iter()
                .filter_map(|id| Problem::new(id).ok())
                .collect()
        }))
    }

    pub async fn put(&self, user: &User, problems: &[Problem]) -> Result<(), JsValue> {
        let cache = ProblemCache::new(user, problems);
        self.db.put(Self::STORE_NAME, &cache).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProblemCache {
    user_id: String,
    problem_ids: Vec<String>,
    fetched_at: i64, // Unix timestamp (seconds)
}

impl ProblemCache {
    const TTL_SECONDS: i64 = 60 * 60; // 1 hour

    pub fn new(user: &User, problems: &[Problem]) -> Self {
        let now = js_sys::Date::now() as i64 / 1000;

        Self {
            user_id: user.id.clone(),
            problem_ids: problems.iter().map(|p| p.id.clone()).collect(),
            fetched_at: now,
        }
    }

    pub fn is_fresh(&self) -> bool {
        let now = js_sys::Date::now() as i64 / 1000;
        now - self.fetched_at <= Self::TTL_SECONDS
    }
}

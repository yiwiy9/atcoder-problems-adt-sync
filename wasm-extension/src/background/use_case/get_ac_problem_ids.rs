use crate::background::api;
use crate::background::database::ProblemCacheDb;
use crate::log;
use crate::models::User;
use wasm_bindgen::prelude::*;

pub async fn handle(user_id: String) -> Result<Vec<String>, JsValue> {
    let user = User::new(&user_id);

    let problem_cache_db = ProblemCacheDb::new().await?;

    if let Some(problems) = problem_cache_db.get(&user).await? {
        log::info(format!("Cache found for user {}", &user.id));
        return Ok(problems.into_iter().map(|p| p.id).collect());
    }

    let problems = api::fetch_ac_problems(&user).await?;

    problem_cache_db.put(&user, &problems).await?;
    log::info(format!("Cache created for user {}", &user.id));

    Ok(problems.iter().map(|p| p.id.clone()).collect())
}

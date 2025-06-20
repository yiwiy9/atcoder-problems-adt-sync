use crate::error::ErrorResponse;
use crate::extractors::VerifiedExtension;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use ddb_client::{DdbError, DdbService};
use lambda_http::tracing;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response {
    problem_ids: Vec<String>,
}

/// Handles GET /users/{user_id}/problems
/// Returns a list of AC problems for the given user.
pub async fn get_ac_problems(
    VerifiedExtension: VerifiedExtension,
    Path(user_id): Path<String>,
    State(ddb_service): State<DdbService>,
) -> Result<impl IntoResponse, ErrorResponse> {
    tracing::info!("Received request for user_id: {}", user_id);

    let ac_problems = match ddb_service.get_user_ac_problems(&user_id).await {
        Ok(record) => record.ac_problems,
        Err(DdbError::NotFound) => {
            tracing::warn!("User ID {} not found", user_id);
            Vec::new()
        }
        Err(err) => {
            tracing::error!("Failed to fetch user AC problems: {}", err);
            return Err(ErrorResponse::internal());
        }
    };

    tracing::info!(
        "Found {} AC problems for user_id: {}",
        ac_problems.len(),
        user_id
    );

    Ok(Json(Response {
        problem_ids: ac_problems,
    }))
}

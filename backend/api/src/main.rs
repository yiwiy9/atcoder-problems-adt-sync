mod constants;
mod error;
mod extractors;
mod handlers;

use axum::{
    Router,
    http::{HeaderValue, header},
    routing::get,
};
use constants::{DYNAMODB_TABLE_ENV, EXTENSION_ORIGIN_ENV, X_EXTENSION_NAME_HEADER};
use ddb_client::DdbService;
use handlers::get_ac_problems;
use lambda_http::{Error, http::Method, run, tracing};
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    // Load DynamoDB table name and initialize service
    let table_name = env::var(DYNAMODB_TABLE_ENV)
        .unwrap_or_else(|_| panic!("Environment variable {} is not set", DYNAMODB_TABLE_ENV));
    let ddb_service = DdbService::from_env(table_name).await;

    // Set up CORS layer with multiple origins
    let extension_origins = env::var(EXTENSION_ORIGIN_ENV)
        .unwrap_or_else(|_| panic!("Environment variable {} is not set", EXTENSION_ORIGIN_ENV));

    let allowed_origins: Vec<HeaderValue> = extension_origins
        .split(',')
        .map(|origin| origin.trim())
        .filter(|origin| !origin.is_empty())
        .map(|origin| origin.parse::<HeaderValue>().unwrap())
        .collect();

    let cors_layer = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET])
        .allow_headers([
            header::CONTENT_TYPE,
            header::HeaderName::from_static(X_EXTENSION_NAME_HEADER),
        ]);

    // Setup router
    let app = Router::new()
        .route("/users/{user_id}/problems", get(get_ac_problems))
        .layer(cors_layer)
        .with_state(ddb_service);

    run(app).await
}

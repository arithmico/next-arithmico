use axum::{
    Router,
    routing::{post, put},
};
use tower_http::trace::TraceLayer;

use crate::state::ServiceState;

pub mod config;
mod error;
mod report_features;
pub mod state;
mod webhook;

pub fn router(state: ServiceState) -> Router {
    Router::new()
        .route("/github/webhook", post(webhook::handler))
        .route("/github/tag/features", put(report_features::handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

use axum::{Router, routing::post};
use tower_http::trace::TraceLayer;

use crate::state::ServiceState;

pub mod config;
mod database;
pub mod state;
mod webhook;

pub fn router(state: ServiceState) -> Router {
    Router::new()
        .route("/github/webhook", post(webhook::handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

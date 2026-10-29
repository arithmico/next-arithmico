use axum::{Router, routing::post};

use crate::state::ServiceState;

pub mod config;
pub mod state;
mod webhook;

pub fn router(state: ServiceState) -> Router {
    Router::new()
        .route("/github/webhook", post(webhook::handler))
        .with_state(state)
}

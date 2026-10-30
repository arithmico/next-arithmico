use axum::{Router, routing::post};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::state::ServiceState;

pub mod config;
pub mod state;
mod webhook;

pub fn router(state: ServiceState) -> Router {
    Router::new()
        .route("/github/webhook", post(webhook::handler))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}

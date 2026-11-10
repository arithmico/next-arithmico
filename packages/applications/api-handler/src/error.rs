use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::{error, instrument};

#[derive(Debug, Error)]
#[error("ServiceError")]
pub enum ServiceError {
    InternalServerError(#[from] anyhow::Error),
    NotFound,
    BadRequest,
}

impl IntoResponse for ServiceError {
    #[instrument]
    fn into_response(self) -> axum::response::Response {
        match self {
            ServiceError::InternalServerError(error) => {
                error!(message = "Internal server error", error = ?error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ServiceError::NotFound => StatusCode::NOT_FOUND.into_response(),
            ServiceError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

impl From<database::error::Error> for ServiceError {
    fn from(value: database::error::Error) -> Self {
        Self::InternalServerError(value.into())
    }
}

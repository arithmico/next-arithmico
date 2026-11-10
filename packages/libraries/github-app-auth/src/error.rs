use std::time::SystemTimeError;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("GithubAppAuthError")]
pub enum Error {
    SystemTimeError(#[from] SystemTimeError),
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    HttpError(#[from] reqwest::Error),
    SerdeError(#[from] serde_json::Error),
    MissingGithubAppSecret,
}

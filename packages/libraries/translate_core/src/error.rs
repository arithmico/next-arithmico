use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TranslationError {
    #[error("Missing translation key: {0}")]
    MissingKey(String),
}

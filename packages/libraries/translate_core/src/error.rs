use common::Language;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
pub enum TranslationError {
    #[error("Missing translation key: {0}")]
    MissingKey(String),

    #[error("Missing translation for language {0:?}")]
    MissingTranslation(Language),
}

use common::Language;
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum TranslationError {
    #[error("Missing translation key: {0}")]
    MissingKey(String),

    #[error("Missing translation for language {0:?}")]
    MissingTranslation(Language),
}

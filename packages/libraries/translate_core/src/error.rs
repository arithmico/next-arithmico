use thiserror::Error;

use crate::Language;

#[derive(Clone, Debug, Error)]
pub enum TranslationError {
    #[error("Missing translation key: {0}")]
    MissingKey(String),

    #[error("Missing translation template for language {0:?}")]
    MissingTranslationTemplate(Language),
}

use thiserror::Error;
use trace::{Tracable, Trace};
use translate_core::{Translatable, TranslatedMessage};

#[derive(Debug, Clone, Error)]
#[error("ValidationError")]
pub struct Error {
    pub trace: Trace,
    pub message: TranslatedMessage,
}

impl Error {
    pub(crate) fn new(node: impl Tracable, message: TranslatedMessage) -> Self {
        Self {
            trace: node.trace().clone(),
            message,
        }
    }
}

impl Translatable for Error {
    fn translate(
        &self,
        language: translate_core::Language,
    ) -> Result<String, translate_core::TranslationError> {
        self.message.translate(language)
    }
}

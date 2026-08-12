use std::collections::HashMap;

use common::Language;

use crate::{Translatable, TranslationError};

#[derive(Debug, Clone, PartialEq)]
pub struct RenderedTranslatedMessage {
    messages: HashMap<Language, Result<String, TranslationError>>,
}

impl RenderedTranslatedMessage {
    pub(crate) fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub(crate) fn add_message(
        &mut self,
        language: Language,
        message: Result<String, TranslationError>,
    ) {
        self.messages.insert(language, message);
    }
}

impl Translatable for RenderedTranslatedMessage {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        self.messages
            .get(&language)
            .cloned()
            .ok_or(TranslationError::MissingTranslation(language))
            .flatten()
    }
}

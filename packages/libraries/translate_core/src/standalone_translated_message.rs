use std::collections::HashMap;

use crate::{Language, Translatable, TranslationError, TranslationTemplate};

#[derive(Debug, Clone, PartialEq)]
pub struct StandaloneTranslatedMessage {
    template: TranslationTemplate,
    keys: HashMap<String, String>,
}

impl StandaloneTranslatedMessage {
    pub fn new() -> Self {
        Self {
            template: TranslationTemplate::new(),
            keys: HashMap::new(),
        }
    }

    pub fn add_translation(
        &mut self,
        language: Language,
        template: impl ToString,
    ) {
        self.template.add_translation(language, template);
    }

    pub fn translation(
        mut self,
        language: Language,
        template: impl ToString,
    ) -> Self {
        self.add_translation(language, template);
        self
    }

    pub fn add_key(&mut self, key: impl ToString, value: impl ToString) {
        self.keys.insert(key.to_string(), value.to_string());
    }

    pub fn key(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.add_key(key, value);
        self
    }
}

impl Translatable for StandaloneTranslatedMessage {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        self.template.translate_with(language, &self.keys)
    }
}

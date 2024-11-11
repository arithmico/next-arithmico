use std::collections::HashMap;

use crate::{
    Language, Translatable, TranslationError, TranslationTemplateCollection,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TranslatedMessage {
    templates: TranslationTemplateCollection,
    keys: HashMap<String, String>,
}

impl TranslatedMessage {
    pub fn new() -> Self {
        Self {
            templates: TranslationTemplateCollection::new(),
            keys: HashMap::new(),
        }
    }

    pub fn add_translation(
        &mut self,
        language: Language,
        template: impl ToString,
    ) {
        self.templates.add_translation(language, template);
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

impl Translatable for TranslatedMessage {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        self.templates.translate_with(language, &self.keys)
    }
}

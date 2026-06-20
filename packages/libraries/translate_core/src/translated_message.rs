use std::{collections::HashMap, rc::Rc};

use language::Language;

use crate::{Translatable, TranslationError, TranslationTemplateProvider};

#[derive(Debug, Clone)]
pub struct TranslatedMessage {
    template_id: String,
    keys: HashMap<String, String>,
    provider_resolver: fn() -> Rc<TranslationTemplateProvider>,
}

impl PartialEq for TranslatedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.template_id == other.template_id && self.keys == other.keys
    }
}

impl TranslatedMessage {
    pub fn new(
        id: impl ToString,
        resolver: fn() -> Rc<TranslationTemplateProvider>,
    ) -> Self {
        Self {
            template_id: id.to_string(),
            keys: HashMap::new(),
            provider_resolver: resolver,
        }
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
        let provider = (self.provider_resolver)();
        let template = provider.get_template(&self.template_id).ok_or(
            TranslationError::MissingTranslationTemplate(language.clone()),
        )?;
        template.translate_with(language, &self.keys)
    }
}

use std::collections::HashMap;

use crate::{language::Language, translation_template::TranslationTemplate};

#[derive(Debug, Clone, PartialEq)]
pub struct TranslatedMessage {
    map: HashMap<Language, TranslationTemplate>,
}

impl TranslatedMessage {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_translation<T: ToString>(
        &mut self,
        language: Language,
        template: T,
    ) {
        self.map
            .insert(language, TranslationTemplate::new(template));
    }

    pub fn translation<T: ToString>(
        mut self,
        language: Language,
        template: T,
    ) -> Self {
        self.add_translation(language, template);
        self
    }
}

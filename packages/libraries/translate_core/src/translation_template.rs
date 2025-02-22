use std::collections::HashMap;

use crate::{TranslationError, language::Language, template::Template};

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationTemplate {
    templates: HashMap<Language, Template>,
}

impl TranslationTemplate {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn add_translation<T: ToString>(
        &mut self,
        language: Language,
        template: T,
    ) {
        self.templates.insert(language, Template::new(template));
    }

    pub fn translation<T: ToString>(
        mut self,
        language: Language,
        template: T,
    ) -> Self {
        self.add_translation(language, template);
        self
    }

    fn get_template_for(
        &self,
        language: Language,
    ) -> Result<&Template, TranslationError> {
        if let Some(template) = self.templates.get(&language) {
            Ok(template)
        } else {
            Err(TranslationError::MissingTranslationTemplate(language))
        }
    }

    pub fn translate_with(
        &self,
        language: Language,
        keys: &HashMap<String, String>,
    ) -> Result<String, TranslationError> {
        let template = self.get_template_for(language)?;
        template.render_with(&keys)
    }

    pub fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        let template = self.get_template_for(language)?;
        template.render()
    }
}

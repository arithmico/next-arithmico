use leptos::prelude::{Get, Signal};
use translate_core::{Language, TranslationTemplateProvider};

#[derive(Debug, Clone)]
pub struct TranslateContext {
    current_language: Signal<Language>,
    fallback_language: Language,
    translations: TranslationTemplateProvider,
}

impl TranslateContext {
    pub fn new(
        current_language: Signal<Language>,
        fallback_language: Language,
        translations: TranslationTemplateProvider,
    ) -> Self {
        Self {
            current_language,
            fallback_language,
            translations,
        }
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language.get()
    }

    pub fn get_fallback_language(&self) -> Language {
        self.fallback_language
    }

    pub fn get_template_provider(&self) -> &TranslationTemplateProvider {
        &self.translations
    }
}

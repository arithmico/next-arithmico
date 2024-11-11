use leptos::{Signal, SignalGet};
use translate_core::Language;

#[derive(Debug, Clone)]
pub struct TranslateContext {
    current_language: Signal<Language>,
    fallback_language: Language,
}

impl TranslateContext {
    pub fn new(
        current_language: Signal<Language>,
        fallback_language: Language,
    ) -> Self {
        Self {
            current_language,
            fallback_language,
        }
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language.get()
    }

    pub fn get_fallback_language(&self) -> Language {
        self.fallback_language.clone()
    }
}

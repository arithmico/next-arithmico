use std::rc::Rc;

use translate_core::TranslationTemplateProvider;

thread_local! {
    static TRANSLATIONS: Rc<TranslationTemplateProvider> = Rc::new(
        TranslationTemplateProvider::try_from_toml(include_str!("../../translations.toml"))
        .unwrap_or(TranslationTemplateProvider::new())
    );
}

pub fn translation_resolver() -> Rc<TranslationTemplateProvider> {
    TRANSLATIONS.with(|provider| provider.clone())
}

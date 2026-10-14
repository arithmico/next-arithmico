#[macro_export]
macro_rules! configure_translation_resolver {
    ($path:expr) => {
        thread_local! {
            static TRANSLATIONS: std::rc::Rc<$crate::TranslationTemplateProvider>  = std::rc::Rc::new(
                $crate::TranslationTemplateProvider::try_from_toml(include_str!($path))
                .unwrap_or($crate::TranslationTemplateProvider::new())
            );
        }

        pub fn translation_resolver() -> std::rc::Rc<$crate::TranslationTemplateProvider> {
            TRANSLATIONS.with(|provider| provider.clone())
        }
    };
}

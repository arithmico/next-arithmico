pub trait IntoTranslationId {
    fn into_translation_id(&self) -> &str;
}

impl IntoTranslationId for String {
    fn into_translation_id(&self) -> &str {
        self.as_str()
    }
}

impl IntoTranslationId for &str {
    fn into_translation_id(&self) -> &str {
        self
    }
}

pub trait GetTranslationId {
    fn get_translation_id(&self) -> &str;
}

impl GetTranslationId for String {
    fn get_translation_id(&self) -> &str {
        self.as_str()
    }
}

impl GetTranslationId for &str {
    fn get_translation_id(&self) -> &str {
        self
    }
}

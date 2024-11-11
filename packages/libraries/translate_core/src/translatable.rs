use crate::{Language, TranslationError};

pub trait Translatable {
    fn translate(&self, language: Language)
        -> Result<String, TranslationError>;
}

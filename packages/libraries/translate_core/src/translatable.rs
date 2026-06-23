use language::Language;

use crate::TranslationError;

pub trait Translatable {
    fn translate(&self, language: Language)
    -> Result<String, TranslationError>;
}

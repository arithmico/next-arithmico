use language::Language;

use crate::TranslationError;

pub trait Translatable {
    fn translate(&self, language: Language)
    -> Result<String, TranslationError>;

    fn quoted(&self, language: Language) -> Result<String, TranslationError> {
        match language {
            Language::German => {
                Ok(format!(r#""{}""#, self.translate(language)?))
            }
            Language::English => {
                Ok(format!(r#"'{}'"#, self.translate(language)?))
            }
        }
    }
}

pub trait Quoted {
    fn quoted(&self, language: Language) -> Result<String, TranslationError>;
}

impl<T: ToString> Quoted for T {
    fn quoted(&self, language: Language) -> Result<String, TranslationError> {
        match language {
            Language::German => Ok(format!(r#""{}""#, self.to_string())),
            Language::English => Ok(format!(r#"'{}'"#, self.to_string())),
        }
    }
}

pub trait TranslatableList {
    fn translate_list_or(
        &self,
        language: Language,
    ) -> Result<String, TranslationError>;

    fn translate_list_and(
        &self,
        language: Language,
    ) -> Result<String, TranslationError>;
}

impl<T: Translatable> TranslatableList for [T] {
    fn translate_list_or(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        match self {
            [] => Ok(String::new()),
            [item] => item.translate(language),
            [head @ .., last] => {
                let mut output = head
                    .into_iter()
                    .map(|t| t.translate(language))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(match language {
                        Language::German => ", ",
                        Language::English => ", ",
                    });
                output.push_str(match language {
                    Language::German => " oder ",
                    Language::English => " or ",
                });
                output.push_str(&last.translate(language)?);
                Ok(output)
            }
        }
    }

    fn translate_list_and(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        match self {
            [] => Ok(String::new()),
            [item] => item.translate(language),
            [head @ .., last] => {
                let mut output = head
                    .into_iter()
                    .map(|t| t.translate(language))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(match language {
                        Language::German => ", ",
                        Language::English => ", ",
                    });
                output.push_str(match language {
                    Language::German => " und ",
                    Language::English => " and ",
                });
                output.push_str(&last.translate(language)?);
                Ok(output)
            }
        }
    }
}

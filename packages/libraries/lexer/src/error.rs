use common::Language;
use thiserror::Error;
use translate_core::{Translatable, TranslatedMessage, TranslationError};

use crate::{Position, translations::translation_resolver};

#[derive(Debug, Clone, Error, PartialEq)]
pub enum Error {
    #[error("UnexpectedCharacter")]
    UnexpectedCharacter { character: char, position: Position },

    #[error("InvalidLeadingZero")]
    InvalidLeadingZero { position: Position },

    #[error("MissingDecimalPlaces")]
    MissingDecimalPlaces { position: Position },
}

impl Translatable for Error {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        match self {
            Error::UnexpectedCharacter {
                character,
                position,
            } => TranslatedMessage::new(
                "error.unexpected_character",
                translation_resolver,
            )
            .key("character", character.to_string())
            .key("position", (position.char_index + 1).to_string())
            .translate(language),
            Error::InvalidLeadingZero { position } => TranslatedMessage::new(
                "error.invalid_leading_zero",
                translation_resolver,
            )
            .key("position", (position.char_index + 1).to_string())
            .translate(language),
            Error::MissingDecimalPlaces { position } => TranslatedMessage::new(
                "error.missing_decimal_places",
                translation_resolver,
            )
            .key("position", (position.char_index + 1).to_string())
            .translate(language),
        }
    }
}

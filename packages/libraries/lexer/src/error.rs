use thiserror::Error;

use crate::Position;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum Error {
    #[error("UnexpectedCharacter")]
    UnexpectedCharacter { character: char, position: Position },

    #[error("InvalidLeadingZero")]
    InvalidLeadingZero { position: Position },

    #[error("MissingDecimalPlaces")]
    MissingDecimalPlaces { position: Position },
}

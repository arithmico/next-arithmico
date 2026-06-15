use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("UnexpectedCharacter")]
    UnexpectedCharacter(char),
}

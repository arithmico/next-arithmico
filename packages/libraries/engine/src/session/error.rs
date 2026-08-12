use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("ParseError: {0:?}")]
    ParseNodeError(#[from] parser::Error),

    #[error("SerializeError: {0:?}")]
    SerializeNodeError(#[from] serializer::Error),

    #[error("EvaluateError: {0:?}")]
    EvaluateNodeError(#[from] evaluator::Error),
}

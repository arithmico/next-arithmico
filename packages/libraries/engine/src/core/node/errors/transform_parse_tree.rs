use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum TransformParseTreeError {
    #[error("conversion failed")]
    ConversionFailed,
}

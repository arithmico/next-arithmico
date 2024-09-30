use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum SerializeNodeError {
    #[error("unsupported node")]
    UnsupportedNode,

    #[error("invalid node")]
    InvalidNode,
}

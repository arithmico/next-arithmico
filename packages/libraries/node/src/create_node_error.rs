use thiserror::Error;

use crate::NodeType;

#[derive(Debug, Clone, Copy, Error)]
#[error("Failed to create node")]
pub struct CreateNodeError {
    pub node_type: NodeType,
}

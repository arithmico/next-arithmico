use node::{GetNodeType, Node, NodeType};
use thiserror::Error;
use trace::{Span, Tracable, Trace};

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
    #[error("unsupported node type {node_type}")]
    UnsupportedNode { node_type: NodeType, trace: Trace },
    #[error("malformed node of type {node_type}")]
    MalformedNode { node_type: NodeType, trace: Trace },
}

impl Error {
    pub fn unsupported_node(node: &Node) -> Self {
        Self::UnsupportedNode {
            node_type: node.node_type(),
            trace: node.trace().clone(),
        }
    }

    pub fn malformed_node(node: &Node) -> Self {
        Self::MalformedNode {
            node_type: node.node_type(),
            trace: node.trace().clone(),
        }
    }

    pub fn get_spans(&self) -> impl Iterator<Item = Span> {
        match self {
            Error::UnsupportedNode { trace, .. } => trace.first_spans(),
            Error::MalformedNode { trace, .. } => trace.first_spans(),
        }
    }
}

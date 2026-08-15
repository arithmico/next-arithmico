use std::ops::Rem;

use trace::Trace;

use crate::{CreateNodeError, Node, NodeType, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct DataFrame {
    pub headers: Vec<Option<Node>>,
    pub data: Vec<Option<Node>>,
    pub trace: Trace,
}

impl DataFrame {
    pub fn new(
        headers: Vec<Option<Node>>,
        data: Vec<Option<Node>>,
    ) -> Result<Self, CreateNodeError> {
        if headers.len() == 0 || data.len().rem(headers.len()) != 0 {
            return Err(CreateNodeError {
                node_type: NodeType::DataFrame,
            });
        }
        Ok(Self {
            headers,
            data,
            trace: Trace::new(),
        })
    }

    pub fn new_node(
        headers: Vec<Option<Node>>,
        data: Vec<Option<Node>>,
    ) -> Result<Node, CreateNodeError> {
        Ok(Node::DataFrame(Self::new(headers, data)?))
    }
}

impl_node_traits!(DataFrame);

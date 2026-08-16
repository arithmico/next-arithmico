use std::ops::Rem;

use thiserror::Error;
use trace::Trace;

use crate::{CreateNodeError, Node, NodeType, impl_node_traits};
#[derive(Debug, Clone, Copy, Error)]
#[error("PushRowError")]
pub struct PushRowError;

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

    pub fn push_row(
        &mut self,
        mut row: Vec<Option<Node>>,
    ) -> Result<(), PushRowError> {
        if row.len() != self.headers.len() {
            return Err(PushRowError);
        }
        self.data.append(&mut row);
        Ok(())
    }

    pub fn headers_empty(&self) -> bool {
        !self.headers.iter().any(|header| header.is_some())
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Option<Node>]> {
        self.data.chunks(self.headers.len())
    }

    pub fn row(&self, index: usize) -> &[Option<Node>] {
        let row_len = self.headers.len();
        &self.data[(index * row_len)..((index + 1) * row_len)]
    }
}

impl_node_traits!(DataFrame);

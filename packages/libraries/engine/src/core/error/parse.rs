use std::collections::VecDeque;

use nom::error::ErrorKind;
use thiserror::Error;

mod context_error;
pub mod map_parse_error;
mod parse_error;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("ParserError")]
pub enum ParseNodeError {
    Leaf {
        kind: ErrorKind,
        input: String,
    },
    LeafWithExpectation {
        input: String,
        expectation: String,
    },
    MissingParenthesis {
        round: i64,
        square: i64,
    },
    Node {
        children: Vec<ParseNodeError>,
    },
    Context {
        context: Vec<ErrorContext>,
        inner: Box<ParseNodeError>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorContext {
    pub context: String,
    pub input: String,
}

impl ErrorContext {
    pub fn new(context: &str, input: &str) -> Self {
        Self {
            context: context.to_string(),
            input: input.to_string(),
        }
    }
}

impl ParseNodeError {
    pub fn new_leaf(input: &str, kind: ErrorKind) -> Self {
        Self::Leaf {
            kind,
            input: input.to_string(),
        }
    }

    pub fn new_leaf_with_expectation(input: &str, expectation: &str) -> Self {
        Self::LeafWithExpectation {
            input: input.to_string(),
            expectation: expectation.to_string(),
        }
    }

    pub fn new_node(children: Vec<ParseNodeError>) -> Self {
        Self::Node { children }
    }

    /// Determines how much input was left when the error occurred.
    /// A smaller length means the parser progressed further into the string.
    pub fn remaining_input_len(&self) -> usize {
        match self {
            Self::Leaf { input, .. } => input.len(),
            Self::LeafWithExpectation { input, .. } => input.len(),
            Self::Context { inner, .. } => inner.remaining_input_len(),
            Self::Node { children } => children
                .iter()
                .map(|c| c.remaining_input_len())
                .min()
                .unwrap_or(usize::MAX),
            Self::MissingParenthesis { .. } => usize::MAX,
        }
    }

    /// Gets a reference to the innermost error (ignoring Contexts)
    pub fn base_error(&self) -> &Self {
        // TODO: implement this without recursion
        match self {
            Self::Context { inner, .. } => inner.base_error(),
            _ => self,
        }
    }

    /// Flattens nested Nodes and pushes Contexts down to the leaves.
    /// `Context(A, Node(B, C))` becomes `[Context(A, B), Context(A, C)]`.
    pub fn flatten(self) -> Vec<Self> {
        match self {
            Self::Node { children } => {
                let mut res = Vec::new();
                for child in children {
                    res.extend(child.flatten());
                }
                res
            }
            Self::Context { context, inner } => {
                let inner_flat = inner.flatten();
                inner_flat
                    .into_iter()
                    .map(|child| match child {
                        // If the child is also a context, merge them
                        Self::Context {
                            context: mut child_ctx,
                            inner: child_inner,
                        } => {
                            let mut new_ctx = context.clone();
                            new_ctx.append(&mut child_ctx);
                            Self::Context {
                                context: new_ctx,
                                inner: child_inner,
                            }
                        }
                        // Otherwise wrap the leaf in the context
                        _ => Self::Context {
                            context: context.clone(),
                            inner: Box::new(child),
                        },
                    })
                    .collect()
            }
            _ => vec![self],
        }
    }

    fn expectations(&self) -> Vec<String> {
        let mut expectations = Vec::new();
        let mut queue = VecDeque::<&ParseNodeError>::new();
        queue.push_back(self);
        while let Some(error) = queue.pop_front() {
            match error {
                ParseNodeError::LeafWithExpectation { expectation, .. } => {
                    expectations.push(expectation.clone());
                }
                ParseNodeError::Node { children } => {
                    children.iter().for_each(|item| queue.push_back(item));
                }
                ParseNodeError::Context { inner, .. } => {
                    queue.push_back(inner);
                }
                _ => (),
            }
        }
        expectations
    }

    pub fn summary(&self) -> ParseNodeErrorSummary {
        if let Self::MissingParenthesis { round, square } = self {
            return ParseNodeErrorSummary::MissingParenthesis {
                round: *round,
                square: *square,
            };
        };

        let expectations = self.expectations();

        ParseNodeErrorSummary::Expectation { expectations }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNodeErrorSummary {
    MissingParenthesis { round: i64, square: i64 },
    Expectation { expectations: Vec<String> },
}

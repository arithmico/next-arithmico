use nom::error::{ContextError, ErrorKind, ParseError};
use thiserror::Error;

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
    MissingOpeningParenthesis {
        round: usize,
        square: usize,
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
            Self::MissingOpeningParenthesis { .. } => usize::MAX,
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
}

impl ParseError<&str> for ParseNodeError {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self::new_leaf(input, kind)
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    fn or(self, other: Self) -> Self {
        let self_len = self.remaining_input_len();
        let other_len = other.remaining_input_len();

        if self_len < other_len {
            return self;
        } else if other_len < self_len {
            return other;
        }

        if self == other {
            return self;
        }

        let mut self_leaves = self.flatten();
        let other_leaves = other.flatten();

        // deduplicate strictly by the actual base expectation
        for leaf in other_leaves {
            if !self_leaves
                .iter()
                .any(|existing| existing.base_error() == leaf.base_error())
            {
                self_leaves.push(leaf);
            }
        }

        if self_leaves.len() == 1 {
            self_leaves.pop().unwrap()
        } else {
            Self::Node {
                children: self_leaves,
            }
        }
    }
}

impl ContextError<&str> for ParseNodeError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        if let Self::Context { mut context, inner } = other {
            context.push(ErrorContext::new(ctx, input));
            Self::Context { context, inner }
        } else {
            Self::Context {
                context: vec![ErrorContext::new(ctx, input)],
                inner: Box::new(other),
            }
        }
    }
}

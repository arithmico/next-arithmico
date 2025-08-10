use nom::error::{ContextError, ErrorKind, ParseError};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("ParserError")]
pub enum ParseNodeError {
    Leaf {
        kind: ErrorKind,
    },
    Node {
        children: Vec<ParseNodeError>,
    },
    Context {
        context: Vec<String>,
        inner: Box<ParseNodeError>,
    },
}

impl ParseNodeError {
    fn new_leaf(kind: ErrorKind) -> Self {
        Self::Leaf { kind }
    }

    fn new_node(children: Vec<ParseNodeError>) -> Self {
        Self::Node { children }
    }
}

impl<I> ParseError<I> for ParseNodeError {
    fn from_error_kind(_input: I, kind: nom::error::ErrorKind) -> Self {
        Self::new_leaf(kind)
    }

    fn append(input: I, kind: ErrorKind, other: Self) -> Self {
        if kind == ErrorKind::Alt {
            other
        } else {
            Self::new_node(vec![other, Self::from_error_kind(input, kind)])
        }
    }

    fn or(self, other: Self) -> Self {
        match self {
            ParseNodeError::Leaf { .. } | ParseNodeError::Context { .. } => {
                Self::new_node(vec![self, other])
            }
            ParseNodeError::Node { mut children } => {
                children.push(other);
                Self::Node { children }
            }
        }
    }
}

impl<I> ContextError<I> for ParseNodeError {
    fn add_context(_input: I, ctx: &'static str, other: Self) -> Self {
        if let Self::Context { mut context, inner } = other {
            context.push(ctx.to_string());
            Self::Context { context, inner }
        } else {
            Self::Context {
                context: vec![ctx.to_string()],
                inner: Box::new(other),
            }
        }
    }
}

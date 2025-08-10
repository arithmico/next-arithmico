use nom::error::{ContextError, ErrorKind, ParseError};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("ParserError")]
pub enum ParseNodeError {
    Leaf {
        kind: ErrorKind,
        input: String,
    },
    Node {
        children: Vec<ParseNodeError>,
    },
    Context {
        context: Vec<ErrorContext>,
        inner: Box<ParseNodeError>,
    },
}
#[derive(Debug, Clone)]
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
    fn new_leaf(input: &str, kind: ErrorKind) -> Self {
        Self::Leaf {
            kind,
            input: input.to_string(),
        }
    }

    fn new_node(children: Vec<ParseNodeError>) -> Self {
        Self::Node { children }
    }
}

impl ParseError<&str> for ParseNodeError {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self::new_leaf(input, kind)
    }

    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
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

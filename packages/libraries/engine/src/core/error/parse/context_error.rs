use nom::error::ContextError;

use crate::core::{ErrorContext, ParseNodeError};

impl ContextError<&str> for ParseNodeError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        // ignore context on missing parenthesis errors
        if let Self::MissingParenthesis { .. } = other {
            return other;
        }
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

use nom::error::{ErrorKind, ParseError};

use crate::core::ParseNodeError;

impl ParseError<&str> for ParseNodeError {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self::new_leaf(input, kind)
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    fn or(self, other: Self) -> Self {
        // ignore other erros from missing parenthesis errors
        if let Self::MissingParenthesis { .. } = self {
            return self;
        }
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

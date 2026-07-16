mod arrow;
mod boolean;
mod caret;
mod define;
mod divide;
mod equals;
mod greater_than;
mod greater_than_or_equals;
mod identifier;
mod left_bracket;
mod left_parenthesis;
mod less_than;
mod less_than_or_equals;
mod minus;
mod multiply;
mod number;
mod plus;
mod right_bracket;
mod right_parenthesis;
mod separator;

pub use arrow::*;
pub use boolean::*;
pub use caret::*;
pub use define::*;
pub use divide::*;
pub use equals::*;
pub use greater_than::*;
pub use greater_than_or_equals::*;
pub use identifier::*;
pub use left_bracket::*;
pub use left_parenthesis::*;
pub use less_than::*;
pub use less_than_or_equals::*;
pub use minus::*;
pub use multiply::*;
pub use number::*;
pub use plus::*;
pub use right_bracket::*;
pub use right_parenthesis::*;
pub use separator::*;

use crate::{GetTokenKind, GetTokenSpan};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(IdentifierToken),
    Number(NumberToken),
    Boolean(BooleanToken),
    LeftParenthesis(LeftParenthesisToken),
    RightParenthesis(RightParenthesisToken),
    LeftBracket(LeftBracketToken),
    RightBracket(RightBracketToken),
    Plus(PlusToken),
    Minus(MinusToken),
    Multiply(MultiplyToken),
    Divide(DivideToken),
    Caret(CaretToken),
    Separator(SeparatorToken),
    Arrow(ArrowToken),
    Define(DefineToken),
    LessThan(LessThanToken),
    LessThanOrEquals(LessThanOrEqualsToken),
    GreaterThan(GreaterThanToken),
    GreaterThanOrEquals(GreaterThanOrEqualsToken),
    Equals(EqualsToken),
}

impl GetTokenKind for Token {
    fn token_kind(&self) -> crate::TokenKind {
        match self {
            Token::Identifier(token) => token.token_kind(),
            Token::Number(token) => token.token_kind(),
            Token::Boolean(token) => token.token_kind(),
            Token::LeftParenthesis(token) => token.token_kind(),
            Token::RightParenthesis(token) => token.token_kind(),
            Token::LeftBracket(token) => token.token_kind(),
            Token::RightBracket(token) => token.token_kind(),
            Token::Plus(token) => token.token_kind(),
            Token::Minus(token) => token.token_kind(),
            Token::Multiply(token) => token.token_kind(),
            Token::Divide(token) => token.token_kind(),
            Token::Caret(token) => token.token_kind(),
            Token::Separator(token) => token.token_kind(),
            Token::Arrow(token) => token.token_kind(),
            Token::Define(token) => token.token_kind(),
            Token::LessThan(token) => token.token_kind(),
            Token::LessThanOrEquals(token) => token.token_kind(),
            Token::GreaterThan(token) => token.token_kind(),
            Token::GreaterThanOrEquals(token) => token.token_kind(),
            Token::Equals(token) => token.token_kind(),
        }
    }
}

impl GetTokenSpan for Token {
    fn get_span(&self) -> crate::Span {
        match self {
            Token::Identifier(token) => token.get_span(),
            Token::Number(token) => token.get_span(),
            Token::Boolean(token) => token.get_span(),
            Token::LeftParenthesis(token) => token.get_span(),
            Token::RightParenthesis(token) => token.get_span(),
            Token::LeftBracket(token) => token.get_span(),
            Token::RightBracket(token) => token.get_span(),
            Token::Plus(token) => token.get_span(),
            Token::Minus(token) => token.get_span(),
            Token::Multiply(token) => token.get_span(),
            Token::Divide(token) => token.get_span(),
            Token::Caret(token) => token.get_span(),
            Token::Separator(token) => token.get_span(),
            Token::Arrow(token) => token.get_span(),
            Token::Define(token) => token.get_span(),
            Token::LessThan(token) => token.get_span(),
            Token::LessThanOrEquals(token) => token.get_span(),
            Token::GreaterThan(token) => token.get_span(),
            Token::GreaterThanOrEquals(token) => token.get_span(),
            Token::Equals(token) => token.get_span(),
        }
    }
}

pub trait DowncastToken {
    fn downcast(token: &Token) -> Option<&Self>;
}

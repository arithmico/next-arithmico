mod and;
mod arrow;
mod asterisk;
mod boolean;
mod caret;
mod define;
mod equals;
mod exclamation_mark;
mod greater_than;
mod greater_than_or_equals;
mod identifier;
mod left_bracket;
mod left_parenthesis;
mod less_than;
mod less_than_or_equals;
mod minus;
mod number;
mod or;
mod plus;
mod right_bracket;
mod right_parenthesis;
mod separator;
mod slash;

pub use and::*;
pub use arrow::*;
pub use asterisk::*;
pub use boolean::*;
pub use caret::*;
pub use define::*;
pub use equals::*;
pub use exclamation_mark::*;
pub use greater_than::*;
pub use greater_than_or_equals::*;
pub use identifier::*;
pub use left_bracket::*;
pub use left_parenthesis::*;
pub use less_than::*;
pub use less_than_or_equals::*;
pub use minus::*;
pub use number::*;
pub use or::*;
pub use plus::*;
pub use right_bracket::*;
pub use right_parenthesis::*;
pub use separator::*;
pub use slash::*;

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
    Asterisk(AsteriskToken),
    Slash(SlashToken),
    Caret(CaretToken),
    Separator(SeparatorToken),
    Arrow(ArrowToken),
    Define(DefineToken),
    LessThan(LessThanToken),
    LessThanOrEquals(LessThanOrEqualsToken),
    GreaterThan(GreaterThanToken),
    GreaterThanOrEquals(GreaterThanOrEqualsToken),
    Equals(EqualsToken),
    And(AndToken),
    Or(OrToken),
    ExclamationMark(ExclamationMarkToken),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Identifier(identifier_token) => {
                identifier_token.name.to_string()
            }
            Token::Number(number_token) => number_token.value.to_string(),
            Token::Boolean(boolean_token) => boolean_token.value.to_string(),
            Token::LeftParenthesis(_) => String::from("("),
            Token::RightParenthesis(_) => String::from(")"),
            Token::LeftBracket(_) => String::from("["),
            Token::RightBracket(_) => String::from("]"),
            Token::Plus(_) => String::from("+"),
            Token::Minus(_) => String::from("-"),
            Token::Asterisk(_) => String::from("*"),
            Token::Slash(_) => String::from("/"),
            Token::Caret(_) => String::from("^"),
            Token::Separator(separator_token) => {
                separator_token.content.clone()
            }
            Token::Arrow(_) => String::from("->"),
            Token::Define(_) => String::from(":="),
            Token::LessThan(_) => String::from("<"),
            Token::LessThanOrEquals(_) => String::from("<="),
            Token::GreaterThan(_) => String::from(">"),
            Token::GreaterThanOrEquals(_) => String::from(">="),
            Token::Equals(_) => String::from("="),
            Token::And(_) => String::from("&"),
            Token::Or(_) => String::from("|"),
            Token::ExclamationMark(_) => String::from("!"),
        }
    }
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
            Token::Asterisk(token) => token.token_kind(),
            Token::Slash(token) => token.token_kind(),
            Token::Caret(token) => token.token_kind(),
            Token::Separator(token) => token.token_kind(),
            Token::Arrow(token) => token.token_kind(),
            Token::Define(token) => token.token_kind(),
            Token::LessThan(token) => token.token_kind(),
            Token::LessThanOrEquals(token) => token.token_kind(),
            Token::GreaterThan(token) => token.token_kind(),
            Token::GreaterThanOrEquals(token) => token.token_kind(),
            Token::Equals(token) => token.token_kind(),
            Token::And(token) => token.token_kind(),
            Token::Or(token) => token.token_kind(),
            Token::ExclamationMark(token) => token.token_kind(),
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
            Token::Asterisk(token) => token.get_span(),
            Token::Slash(token) => token.get_span(),
            Token::Caret(token) => token.get_span(),
            Token::Separator(token) => token.get_span(),
            Token::Arrow(token) => token.get_span(),
            Token::Define(token) => token.get_span(),
            Token::LessThan(token) => token.get_span(),
            Token::LessThanOrEquals(token) => token.get_span(),
            Token::GreaterThan(token) => token.get_span(),
            Token::GreaterThanOrEquals(token) => token.get_span(),
            Token::Equals(token) => token.get_span(),
            Token::And(token) => token.get_span(),
            Token::Or(token) => token.get_span(),
            Token::ExclamationMark(token) => token.get_span(),
        }
    }
}

pub trait DowncastToken {
    fn downcast(token: &Token) -> Option<&Self>;
}

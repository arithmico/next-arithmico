use std::range::Range;

use crate::{cursor::LexerCursor, Error, Token, TokenContent};

// TODO: define language once for the whole workspace
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    German,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

pub fn tokenize<'a>(
    input: &'a str,
    language: Language,
) -> Result<Vec<Token<'a>>, Error> {
    let mut tokens = Vec::new();
    let mut cursor = LexerCursor::from(input);
    while let Some((index, char)) = cursor.next() {
        match char {
            // whitespace
            ' ' | '\t' | '\n' | '\r' => {
                continue;
            }
            '+' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Plus,
                });
            }
            '-' if cursor.match_and_advance(">") => {
                tokens.push(Token {
                    span: Range::from(index..index + 2),
                    content: TokenContent::Arrow,
                });
            }
            '-' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Minus,
                });
            }
            '*' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Multiply,
                });
            }
            '/' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Divide,
                });
            }
            '^' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Caret,
                });
            }
            ':' if cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Range::from(index..index + 2),
                    content: TokenContent::Define,
                });
            }
            '<' if cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Range::from(index..index + 2),
                    content: TokenContent::LessThanOrEquals,
                });
            }
            '<' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::LessThan,
                });
            }
            '>' if cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Range::from(index..index + 2),
                    content: TokenContent::GreaterThanOrEquals,
                });
            }
            '>' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::GreaterThan,
                });
            }
            ',' if language == Language::English => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Separator,
                });
            }
            ';' if language == Language::German => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::Separator,
                });
            }
            '(' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::LeftParenthesis,
                });
            }
            ')' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::RightParenthesis,
                });
            }
            '[' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::LeftBracket,
                });
            }
            ']' => {
                tokens.push(Token {
                    span: Range::from(index..index + 1),
                    content: TokenContent::RightBracket,
                });
            }
            // true
            't' if cursor.match_and_advance("rue") => {
                tokens.push(Token {
                    span: Range::from(index..index + 4),
                    content: TokenContent::Boolean(true),
                });
            }
            // false
            'f' if cursor.match_and_advance("alse") => {
                tokens.push(Token {
                    span: Range::from(index..index + 5),
                    content: TokenContent::Boolean(false),
                });
            }
            // identifier
            'a'..='z' | 'A'..='Z' | '_' => {
                todo!()
            }
            // number
            '0'..='9' => {
                todo!()
            }
            _ => return Err(Error::UnexpectedCharacter(char)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace() {
        assert_eq!(tokenize("", Default::default()).unwrap(), vec![]);
        assert_eq!(tokenize(" ", Default::default()).unwrap(), vec![]);
        assert_eq!(tokenize("\n", Default::default()).unwrap(), vec![]);
        assert_eq!(tokenize("\r", Default::default()).unwrap(), vec![]);
        assert_eq!(tokenize("\t", Default::default()).unwrap(), vec![]);
    }

    #[test]
    fn plus() {
        assert_eq!(
            tokenize("    +  ", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Plus
            }]
        );
    }

    #[test]
    fn arrow() {
        assert_eq!(
            tokenize(" \n \t-> \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..6),
                content: TokenContent::Arrow
            }]
        );
    }

    #[test]
    fn minus() {
        assert_eq!(
            tokenize(" \n \t- \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Minus
            }]
        );
    }

    #[test]
    fn multiply() {
        assert_eq!(
            tokenize(" \n \t* \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Multiply
            }]
        );
    }

    #[test]
    fn divide() {
        assert_eq!(
            tokenize(" \n \t/ \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Divide
            }]
        );
    }

    #[test]
    fn caret() {
        assert_eq!(
            tokenize(" \n \t^ \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Caret
            }]
        );
    }

    #[test]
    fn define() {
        assert_eq!(
            tokenize(" \n \t:= \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..6),
                content: TokenContent::Define
            }]
        );
    }

    #[test]
    fn less_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t<= \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..6),
                content: TokenContent::LessThanOrEquals
            }]
        );
    }

    #[test]
    fn less_than() {
        assert_eq!(
            tokenize(" \n \t< \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::LessThan
            }]
        );
    }

    #[test]
    fn greater_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t>= \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..6),
                content: TokenContent::GreaterThanOrEquals
            }]
        );
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            tokenize(" \n \t> \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::GreaterThan
            }]
        );
    }

    #[test]
    fn separator() {
        assert_eq!(
            tokenize(" \n \t, \r", Language::English).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Separator
            }]
        );
        assert_eq!(
            tokenize(" \n \t; \r", Language::German).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::Separator
            }]
        );
    }

    #[test]
    fn left_parenthesis() {
        assert_eq!(
            tokenize(" \n \t( \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::LeftParenthesis
            }]
        );
    }

    #[test]
    fn right_parenthesis() {
        assert_eq!(
            tokenize(" \n \t) \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::RightParenthesis
            }]
        );
    }

    #[test]
    fn left_bracket() {
        assert_eq!(
            tokenize(" \n \t[ \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::LeftBracket
            }]
        );
    }

    #[test]
    fn right_bracket() {
        assert_eq!(
            tokenize(" \n \t] \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..5),
                content: TokenContent::RightBracket
            }]
        );
    }

    #[test]
    fn boolean_true() {
        assert_eq!(
            tokenize(" \n \ttrue \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..8),
                content: TokenContent::Boolean(true)
            }]
        );
    }

    #[test]
    fn boolean_false() {
        assert_eq!(
            tokenize(" \n \tfalse \r", Default::default()).unwrap(),
            vec![Token {
                span: Range::from(4..9),
                content: TokenContent::Boolean(false)
            }]
        );
    }
}

use crate::{cursor::LexerCursor, Error, Span, Token, TokenContent};

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
    while let Some((position, char)) = cursor.next() {
        match char {
            // whitespace
            ' ' | '\t' | '\n' | '\r' => {
                continue;
            }
            '+' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Plus,
                });
            }
            '-' if let Some(span) = cursor.match_and_advance(">") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::Arrow,
                });
            }
            '-' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Minus,
                });
            }
            '*' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Multiply,
                });
            }
            '/' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Divide,
                });
            }
            '^' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Caret,
                });
            }
            ':' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::Define,
                });
            }
            '<' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::LessThanOrEquals,
                });
            }
            '<' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::LessThan,
                });
            }
            '>' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::GreaterThanOrEquals,
                });
            }
            '>' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::GreaterThan,
                });
            }
            ',' if language == Language::English => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Separator,
                });
            }
            ';' if language == Language::German => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::Separator,
                });
            }
            '(' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::LeftParenthesis,
                });
            }
            ')' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::RightParenthesis,
                });
            }
            '[' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::LeftBracket,
                });
            }
            ']' => {
                tokens.push(Token {
                    span: Span::new(position, position),
                    content: TokenContent::RightBracket,
                });
            }
            // true
            't' if let Some(span) = cursor.match_and_advance("rue") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::Boolean(true),
                });
            }
            // false
            'f' if let Some(span) = cursor.match_and_advance("alse") => {
                tokens.push(Token {
                    span: Span::new(position, span.to),
                    content: TokenContent::Boolean(false),
                });
            }
            // identifier
            'a'..='z' | 'A'..='Z' | '_' => {
                let span = if let Some(span) =
                    cursor.match_many_of_and_advance(&[
                        'a'..='z',
                        'A'..='Z',
                        '_'..='_',
                    ]) {
                    Span::new(position, span.to)
                } else {
                    Span::new(position, position)
                };
                tokens.push(Token {
                    span,
                    content: TokenContent::Identifier(
                        span.extract_substr(input),
                    ),
                });
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
    use crate::Position;

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
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Plus
            }]
        );
    }

    #[test]
    fn arrow() {
        assert_eq!(
            tokenize(" \n \t-> \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 5,
                        char_index: 5
                    }
                ),
                content: TokenContent::Arrow
            }]
        );
    }

    #[test]
    fn minus() {
        assert_eq!(
            tokenize(" \n \t- \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Minus
            }]
        );
    }

    #[test]
    fn multiply() {
        assert_eq!(
            tokenize(" \n \t* \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Multiply
            }]
        );
    }

    #[test]
    fn divide() {
        assert_eq!(
            tokenize(" \n \t/ \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Divide
            }]
        );
    }

    #[test]
    fn caret() {
        assert_eq!(
            tokenize(" \n \t^ \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Caret
            }]
        );
    }

    #[test]
    fn define() {
        assert_eq!(
            tokenize(" \n \t:= \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 5,
                        char_index: 5
                    }
                ),
                content: TokenContent::Define
            }]
        );
    }

    #[test]
    fn less_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t<= \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 5,
                        char_index: 5
                    }
                ),
                content: TokenContent::LessThanOrEquals
            }]
        );
    }

    #[test]
    fn less_than() {
        assert_eq!(
            tokenize(" \n \t< \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::LessThan
            }]
        );
    }

    #[test]
    fn greater_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t>= \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 5,
                        char_index: 5
                    }
                ),
                content: TokenContent::GreaterThanOrEquals
            }]
        );
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            tokenize(" \n \t> \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::GreaterThan
            }]
        );
    }

    #[test]
    fn separator() {
        assert_eq!(
            tokenize(" \n \t, \r", Language::English).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Separator
            }]
        );
        assert_eq!(
            tokenize(" \n \t; \r", Language::German).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Separator
            }]
        );
    }

    #[test]
    fn left_parenthesis() {
        assert_eq!(
            tokenize(" \n \t( \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::LeftParenthesis
            }]
        );
    }

    #[test]
    fn right_parenthesis() {
        assert_eq!(
            tokenize(" \n \t) \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::RightParenthesis
            }]
        );
    }

    #[test]
    fn left_bracket() {
        assert_eq!(
            tokenize(" \n \t[ \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::LeftBracket
            }]
        );
    }

    #[test]
    fn right_bracket() {
        assert_eq!(
            tokenize(" \n \t] \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::RightBracket
            }]
        );
    }

    #[test]
    fn boolean_true() {
        assert_eq!(
            tokenize(" \n \ttrue \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 7,
                        char_index: 7
                    }
                ),
                content: TokenContent::Boolean(true)
            }]
        );
    }

    #[test]
    fn boolean_false() {
        assert_eq!(
            tokenize(" \n \tfalse \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 8,
                        char_index: 8
                    }
                ),
                content: TokenContent::Boolean(false)
            }]
        );
    }

    #[test]
    fn identifier_length_1() {
        assert_eq!(
            tokenize(" \n \tx \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 4,
                        char_index: 4
                    }
                ),
                content: TokenContent::Identifier("x")
            }]
        );
    }

    #[test]
    fn identifier_length_3() {
        assert_eq!(
            tokenize(" \n \txyz \r", Default::default()).unwrap(),
            vec![Token {
                span: Span::new(
                    Position {
                        byte_index: 4,
                        char_index: 4
                    },
                    Position {
                        byte_index: 6,
                        char_index: 6
                    }
                ),
                content: TokenContent::Identifier("xyz")
            }]
        );
    }
}

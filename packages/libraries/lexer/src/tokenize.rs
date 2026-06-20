use language::Language;

use crate::{cursor::LexerCursor, Error, Span, Token, TokenContent};

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
                // catch invalid leading zeros e. g. "01"
                let head_digits_span =
                    cursor.match_many_of_and_advance(&['0'..='9']);

                match head_digits_span {
                    Some(_) if char == '0' => {
                        return Err(Error::InvalidLeadingZero { position });
                    }
                    head_digits_span => {
                        let head_digits_span = head_digits_span
                            .unwrap_or(Span::new(position, position));

                        let head_digits =
                            Span::new(position, head_digits_span.to)
                                .extract_substr(input);
                        if let Some(decimal_separator_span) = cursor
                            .match_and_advance(match language {
                                Language::German => ",",
                                Language::English => ".",
                            })
                        {
                            match cursor.match_many_of_and_advance(&['0'..='9'])
                            {
                                Some(trailing_digits_span) => {
                                    let span = Span::new(
                                        position,
                                        trailing_digits_span.to,
                                    );
                                    let trailing_digits = trailing_digits_span
                                        .extract_substr(input);
                                    let value: f64 = format!(
                                        "{}.{}",
                                        head_digits, trailing_digits
                                    )
                                    .parse()
                                    .expect("Float");

                                    tokens.push(Token {
                                        span,
                                        content: TokenContent::Number(value),
                                    });
                                }
                                None => {
                                    return Err(Error::MissingDecimalPlaces {
                                        position: decimal_separator_span.to,
                                    })
                                }
                            }
                        } else {
                            let span = Span::new(position, head_digits_span.to);
                            let value: f64 = head_digits
                                .parse()
                                .expect("Multi digit integer");

                            tokens.push(Token {
                                span,
                                content: TokenContent::Number(value),
                            });
                        }
                    }
                }
            }
            _ => {
                return Err(Error::UnexpectedCharacter {
                    character: char,
                    position,
                })
            }
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

    #[test]
    fn number_single_digit_integer() {
        assert_eq!(
            tokenize(" \n \t1 \r", Default::default()).unwrap(),
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
                content: TokenContent::Number(1.0)
            }]
        );
    }

    #[test]
    fn number_multi_digit_integer() {
        assert_eq!(
            tokenize(" \n \t12 \r", Default::default()).unwrap(),
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
                content: TokenContent::Number(12.0)
            }]
        );
    }

    #[test]
    fn number_float_leading_zero() {
        assert_eq!(
            tokenize(" \n \t0.1 \r", Default::default()).unwrap(),
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
                content: TokenContent::Number(0.1)
            }]
        );
    }

    #[test]
    fn number_float_leading_non_zero() {
        assert_eq!(
            tokenize(" \n \t1.12 \r", Default::default()).unwrap(),
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
                content: TokenContent::Number(1.12)
            }]
        );
    }

    #[test]
    fn number_invalid_leading_zero() {
        assert_eq!(
            tokenize(" \n \t01.1 \r", Default::default()),
            Err(Error::InvalidLeadingZero {
                position: Position {
                    byte_index: 4,
                    char_index: 4
                }
            })
        );
    }

    #[test]
    fn number_missing_decimal_places() {
        assert_eq!(
            tokenize(" \n \t1. \r", Default::default()),
            Err(Error::MissingDecimalPlaces {
                position: Position {
                    byte_index: 5,
                    char_index: 5
                }
            })
        );
    }

    #[test]
    fn unexpected_character() {
        assert_eq!(
            tokenize(" \n \t? \r", Default::default()),
            Err(Error::UnexpectedCharacter {
                character: '?',
                position: Position {
                    byte_index: 4,
                    char_index: 4
                }
            })
        );
    }
}

use common::Language;

use crate::{
    ArrowToken, AsteriskToken, BooleanToken, CaretToken, DefineToken, Error,
    GreaterThanOrEqualsToken, GreaterThanToken, IdentifierToken,
    LeftBracketToken, LeftParenthesisToken, LessThanOrEqualsToken,
    LessThanToken, MinusToken, NumberToken, PlusToken, RightBracketToken,
    RightParenthesisToken, SeparatorToken, SlashToken, Span, Token,
    cursor::LexerCursor,
};

pub fn tokenize(input: &str, language: Language) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut cursor = LexerCursor::from(input);
    while let Some((position, char)) = cursor.next() {
        match char {
            // whitespace
            ' ' | '\t' | '\n' | '\r' => {
                continue;
            }
            '+' => {
                tokens.push(Token::Plus(PlusToken {
                    span: Span::new(position, position),
                }));
            }
            '-' if let Some(span) = cursor.match_and_advance(">") => {
                tokens.push(Token::Arrow(ArrowToken {
                    span: Span::new(position, span.to),
                }));
            }
            '-' => {
                tokens.push(Token::Minus(MinusToken {
                    span: Span::new(position, position),
                }));
            }
            '*' => {
                tokens.push(Token::Asterisk(AsteriskToken {
                    span: Span::new(position, position),
                }));
            }
            '/' => {
                tokens.push(Token::Slash(SlashToken {
                    span: Span::new(position, position),
                }));
            }
            '^' => {
                tokens.push(Token::Caret(CaretToken {
                    span: Span::new(position, position),
                }));
            }
            ':' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token::Define(DefineToken {
                    span: Span::new(position, span.to),
                }));
            }
            '<' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token::LessThanOrEquals(LessThanOrEqualsToken {
                    span: Span::new(position, span.to),
                }));
            }
            '<' => {
                tokens.push(Token::LessThan(LessThanToken {
                    span: Span::new(position, position),
                }));
            }
            '>' if let Some(span) = cursor.match_and_advance("=") => {
                tokens.push(Token::GreaterThanOrEquals(
                    GreaterThanOrEqualsToken {
                        span: Span::new(position, span.to),
                    },
                ));
            }
            '>' => {
                tokens.push(Token::GreaterThan(GreaterThanToken {
                    span: Span::new(position, position),
                }));
            }
            ',' if language == Language::English => {
                tokens.push(Token::Separator(SeparatorToken {
                    span: Span::new(position, position),
                    content: String::from(","),
                }));
            }
            ';' if language == Language::German => {
                tokens.push(Token::Separator(SeparatorToken {
                    span: Span::new(position, position),
                    content: String::from(";"),
                }));
            }
            '(' => {
                tokens.push(Token::LeftParenthesis(LeftParenthesisToken {
                    span: Span::new(position, position),
                }));
            }
            ')' => {
                tokens.push(Token::RightParenthesis(RightParenthesisToken {
                    span: Span::new(position, position),
                }));
            }
            '[' => {
                tokens.push(Token::LeftBracket(LeftBracketToken {
                    span: Span::new(position, position),
                }));
            }
            ']' => {
                tokens.push(Token::RightBracket(RightBracketToken {
                    span: Span::new(position, position),
                }));
            }
            '=' => {
                tokens.push(Token::Equals(crate::EqualsToken {
                    span: Span::new(position, position),
                }));
            }
            '&' => {
                tokens.push(Token::And(crate::AndToken {
                    span: Span::new(position, position),
                }));
            }
            '|' => {
                tokens.push(Token::Or(crate::OrToken {
                    span: Span::new(position, position),
                }));
            }
            '!' => {
                tokens.push(Token::ExclamationMark(
                    crate::ExclamationMarkToken {
                        span: Span::new(position, position),
                    },
                ));
            }
            // true
            't' if let Some(span) = cursor.match_and_advance("rue") => {
                tokens.push(Token::Boolean(BooleanToken {
                    span: Span::new(position, span.to),
                    value: true,
                }));
            }
            // false
            'f' if let Some(span) = cursor.match_and_advance("alse") => {
                tokens.push(Token::Boolean(BooleanToken {
                    span: Span::new(position, span.to),
                    value: false,
                }));
            }
            // identifier
            'a'..='z' | 'A'..='Z' | '_' => {
                let span = if let Some(span) =
                    cursor.match_many_of_and_advance(&[
                        'a'..='z',
                        'A'..='Z',
                        '_'..='_',
                        ':'..=':',
                        '0'..='9',
                    ]) {
                    Span::new(position, span.to)
                } else {
                    Span::new(position, position)
                };
                tokens.push(Token::Identifier(IdentifierToken {
                    span,
                    name: span.extract_str_from(input).to_string(),
                }));
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
                                .extract_str_from(input);
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
                                        .extract_str_from(input);
                                    let value: f64 = format!(
                                        "{}.{}",
                                        head_digits, trailing_digits
                                    )
                                    .parse()
                                    .unwrap_or_default();

                                    tokens.push(Token::Number(NumberToken {
                                        span,
                                        value,
                                    }));
                                }
                                None => {
                                    return Err(Error::MissingDecimalPlaces {
                                        position: decimal_separator_span.to,
                                    });
                                }
                            }
                        } else {
                            let span = Span::new(position, head_digits_span.to);
                            let value: f64 =
                                head_digits.parse().unwrap_or_default();

                            tokens.push(Token::Number(NumberToken {
                                span,
                                value,
                            }));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::UnexpectedCharacter {
                    character: char,
                    position,
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{AndToken, ExclamationMarkToken, OrToken, Position};

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
            vec![Token::Plus(PlusToken {
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
            })]
        );
    }

    #[test]
    fn arrow() {
        assert_eq!(
            tokenize(" \n \t-> \r", Default::default()).unwrap(),
            vec![Token::Arrow(ArrowToken {
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
            })]
        );
    }

    #[test]
    fn minus() {
        assert_eq!(
            tokenize(" \n \t- \r", Default::default()).unwrap(),
            vec![Token::Minus(MinusToken {
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
            })]
        );
    }

    #[test]
    fn multiply() {
        assert_eq!(
            tokenize(" \n \t* \r", Default::default()).unwrap(),
            vec![Token::Asterisk(AsteriskToken {
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
            })]
        );
    }

    #[test]
    fn divide() {
        assert_eq!(
            tokenize(" \n \t/ \r", Default::default()).unwrap(),
            vec![Token::Slash(SlashToken {
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
            })]
        );
    }

    #[test]
    fn caret() {
        assert_eq!(
            tokenize(" \n \t^ \r", Default::default()).unwrap(),
            vec![Token::Caret(CaretToken {
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
            })]
        );
    }

    #[test]
    fn define() {
        assert_eq!(
            tokenize(" \n \t:= \r", Default::default()).unwrap(),
            vec![Token::Define(DefineToken {
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
            })]
        );
    }

    #[test]
    fn less_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t<= \r", Default::default()).unwrap(),
            vec![Token::LessThanOrEquals(LessThanOrEqualsToken {
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
            })]
        );
    }

    #[test]
    fn less_than() {
        assert_eq!(
            tokenize(" \n \t< \r", Default::default()).unwrap(),
            vec![Token::LessThan(LessThanToken {
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
            })]
        );
    }

    #[test]
    fn greater_than_or_equals() {
        assert_eq!(
            tokenize(" \n \t>= \r", Default::default()).unwrap(),
            vec![Token::GreaterThanOrEquals(GreaterThanOrEqualsToken {
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
            })]
        );
    }

    #[test]
    fn greater_than() {
        assert_eq!(
            tokenize(" \n \t> \r", Default::default()).unwrap(),
            vec![Token::GreaterThan(GreaterThanToken {
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
            })]
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            tokenize(" \n \t& \r", Default::default()).unwrap(),
            vec![Token::And(AndToken {
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
            })]
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            tokenize(" \n \t| \r", Default::default()).unwrap(),
            vec![Token::Or(OrToken {
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
            })]
        );
    }

    #[test]
    fn separator() {
        assert_eq!(
            tokenize(" \n \t, \r", Language::English).unwrap(),
            vec![Token::Separator(SeparatorToken {
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
                content: String::from(",")
            })]
        );
        assert_eq!(
            tokenize(" \n \t; \r", Language::German).unwrap(),
            vec![Token::Separator(SeparatorToken {
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
                content: String::from(";")
            })]
        );
    }

    #[test]
    fn left_parenthesis() {
        assert_eq!(
            tokenize(" \n \t( \r", Default::default()).unwrap(),
            vec![Token::LeftParenthesis(LeftParenthesisToken {
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
            })]
        );
    }

    #[test]
    fn right_parenthesis() {
        assert_eq!(
            tokenize(" \n \t) \r", Default::default()).unwrap(),
            vec![Token::RightParenthesis(RightParenthesisToken {
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
            })]
        );
    }

    #[test]
    fn left_bracket() {
        assert_eq!(
            tokenize(" \n \t[ \r", Default::default()).unwrap(),
            vec![Token::LeftBracket(LeftBracketToken {
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
            })]
        );
    }

    #[test]
    fn right_bracket() {
        assert_eq!(
            tokenize(" \n \t] \r", Default::default()).unwrap(),
            vec![Token::RightBracket(RightBracketToken {
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
            })]
        );
    }

    #[test]
    fn exclamation_mark() {
        assert_eq!(
            tokenize(" \n \t! \r", Default::default()).unwrap(),
            vec![Token::ExclamationMark(ExclamationMarkToken {
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
            })]
        );
    }

    #[test]
    fn boolean_true() {
        assert_eq!(
            tokenize(" \n \ttrue \r", Default::default()).unwrap(),
            vec![Token::Boolean(BooleanToken {
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
                value: true
            })]
        );
    }

    #[test]
    fn boolean_false() {
        assert_eq!(
            tokenize(" \n \tfalse \r", Default::default()).unwrap(),
            vec![Token::Boolean(BooleanToken {
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
                value: false
            })]
        );
    }

    #[test]
    fn identifier_length_1() {
        assert_eq!(
            tokenize(" \n \tx \r", Default::default()).unwrap(),
            vec![Token::Identifier(IdentifierToken {
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
                name: "x".to_string()
            })]
        );
    }

    #[test]
    fn identifier_length_3() {
        assert_eq!(
            tokenize(" \n \txyz \r", Default::default()).unwrap(),
            vec![Token::Identifier(IdentifierToken {
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
                name: "xyz".to_string()
            })]
        );
    }

    #[test]
    fn identifier_with_colon() {
        assert_eq!(
            tokenize(" \n \tx:y:z \r", Default::default()).unwrap(),
            vec![Token::Identifier(IdentifierToken {
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
                name: "x:y:z".to_string()
            })]
        );
    }

    #[test]
    fn identifier_with_digits() {
        assert_eq!(
            tokenize(" \n \tmu_0 \r", Default::default()).unwrap(),
            vec![Token::Identifier(IdentifierToken {
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
                name: "mu_0".to_string()
            })]
        );
    }

    #[test]
    fn number_single_digit_integer() {
        assert_eq!(
            tokenize(" \n \t1 \r", Default::default()).unwrap(),
            vec![Token::Number(NumberToken {
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
                value: 1.0
            })]
        );
    }

    #[test]
    fn number_multi_digit_integer() {
        assert_eq!(
            tokenize(" \n \t12 \r", Default::default()).unwrap(),
            vec![Token::Number(NumberToken {
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
                value: 12.0
            })]
        );
    }

    #[test]
    fn number_float_leading_zero() {
        assert_eq!(
            tokenize(" \n \t0.1 \r", Default::default()).unwrap(),
            vec![Token::Number(NumberToken {
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
                value: 0.1
            })]
        );
    }

    #[test]
    fn number_float_leading_non_zero() {
        assert_eq!(
            tokenize(" \n \t1.12 \r", Default::default()).unwrap(),
            vec![Token::Number(NumberToken {
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
                value: 1.12
            })]
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

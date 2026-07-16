use lexer::{GetTokenKind, Token, TokenKind};
use node::{
    And, Boolean, Definition, Division, Equals, Function, FunctionCall, FunctionSignature, GetNodeType, GreaterThan, GreaterThanOrEquals, LessThan, LessThanOrEquals, Negate, Node, NodeType, Number, Power, Product, Sum, Symbol, Tensor,
};

use crate::{ParseError::{self, InvalidFunctionArgumentDeclaration, InvalidFunctionName}, ParseResult, cursor::Cursor};

fn binding_power(token: &Token) -> Option<u8> {
    match token.token_kind() {
        TokenKind::Identifier
        | TokenKind::Number
        | TokenKind::Boolean
        | TokenKind::LeftParenthesis
        | TokenKind::RightParenthesis
        | TokenKind::LeftBracket
        | TokenKind::Separator
        | TokenKind::Arrow
        | TokenKind::RightBracket => None,
        TokenKind::Define => Some(0),
        TokenKind::LessThanOrEquals => Some(1),
        TokenKind::LessThan => Some(1),
        TokenKind::GreaterThanOrEquals => Some(1),
        TokenKind::GreaterThan => Some(1),
        TokenKind::Equals => Some(1),
        TokenKind::Plus => Some(2),
        TokenKind::Minus => Some(3),
        TokenKind::Multiply => Some(4),
        TokenKind::Divide => Some(5),
        TokenKind::Caret => Some(6),
    }
}

pub(crate) fn parse_expression<'a>(
    cursor: Cursor<'a>,
) -> ParseResult<'a, Node> {
    parse_expression_pratt(cursor, 0)
}

fn parse_expression_pratt<'a>(
    cursor: Cursor<'a>,
    min_binding_power: u8,
) -> ParseResult<'a, Node> {
    let (mut cursor, mut left) = parse_primary(cursor)?;

    while let Some(next_token) = cursor.peek() {
        let Some(binding_power) = binding_power(next_token) else {
            break;
        };

        if binding_power < min_binding_power {
            break;
        }

        let Some(operator) = cursor.next() else {
            break; // unreachable
        };

        let next_min_minding_power = match operator.token_kind() {
            // right associative operators (e. g. "^")
            TokenKind::Caret => binding_power,
            // left associative operators (e. g. "+", "*", "-", "/")
            _ => binding_power + 1,
        };

        let (next_cursor, right) =
            parse_expression_pratt(cursor, next_min_minding_power)?;
        cursor = next_cursor;

        match operator.token_kind() {
            TokenKind::Plus => {
                if let Node::Sum(sum) = &mut left {
                    sum.elements.push(right);
                } else {
                    left = Sum::new(vec![left, right]);
                }
            }
            TokenKind::Minus => {
                if let Node::Sum(sum) = &mut left {
                    sum.elements.push(Negate::new(right));
                } else {
                    left = Sum::new(vec![left, Negate::new(right)]);
                }
            }
            TokenKind::Multiply => {
                if let Node::Product(product) = &mut left {
                    product.elements.push(right);
                } else {
                    left = Product::new(vec![left, right])
                }
            }
            TokenKind::Divide => {
                left = Division::new(left, right);
            }
            TokenKind::Caret => {
                left = Power::new(left, right);
            }
            TokenKind::Define => {
                if let Node::Symbol(symbol) = left {
                    left = Definition::new(symbol.name, right);
                } else if let Node::FunctionCall(function_call) = &left {
                    let mut signature = FunctionSignature::new();
                    for argument in &function_call.arguments {
                        if let Node::Symbol(symbol) = argument {
                            signature.add_argument(
                                &symbol.name, 
                                |argument| argument.node_type(NodeType::Any)
                            );
                        } else {
                            return Err(InvalidFunctionArgumentDeclaration)
                        }
                    }
                    if let Node::Symbol(symbol) = function_call.target.as_ref() {
                        left = Definition::new(&symbol.name, Function::new(signature, right))
                    } else {
                        return Err(InvalidFunctionName)
                    }
                } else {
                    return Err(ParseError::UnexpectedLeftSideOfDefinition {
                        node_type: left.node_type(),
                    });
                }
            }
            TokenKind::LessThan
            | TokenKind::LessThanOrEquals
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEquals
            | TokenKind::Equals => {
                let mut items = Vec::<(RelationType, Node)>::new();
                // Safety: unwrap can not fail in this context
                let relation_type =
                    RelationType::try_from_token(&operator).unwrap();
                items.push((relation_type, right));
                while 
                    let Some(token) = cursor.peek() &&
                    let Some(relation_type) = RelationType::try_from_token(token)
                {
                    cursor.next();
                    let (next_cursor, item) = parse_expression_pratt(cursor, next_min_minding_power)?;
                    cursor = next_cursor;
                    items.push((relation_type, item));
                }
                let mut output = vec![];
                let mut current_left = left;
                for (relation_type, item) in items {
                    match relation_type {
                        RelationType::Equals => {
                            output.push(Equals::new(current_left, item.clone()));
                            current_left = item;
                        },
                        RelationType::LessThan => {
                            output.push(LessThan::new(current_left, item.clone()));
                            current_left = item;
                        },
                        RelationType::LessThanOrEquals => {
                            output.push(LessThanOrEquals::new(current_left, item.clone()));
                            current_left = item;
                        },
                        RelationType::GreaterThan => {
                            output.push(GreaterThan::new(current_left, item.clone()));
                            current_left = item;
                        },
                        RelationType::GreaterThanOrEquals => {
                            output.push(GreaterThanOrEquals::new(current_left, item.clone()));
                            current_left = item;
                        },
                    }
                }

                if let Some(first) = output.first() && output.len() == 1 {
                    left = first.clone();
                } else {
                    left = And::new(output);
                }
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: vec![
                        TokenKind::Plus,
                        TokenKind::Minus,
                        TokenKind::Multiply,
                        TokenKind::Divide,
                        TokenKind::Caret,
                        TokenKind::Define,
                        TokenKind::LessThan,
                        TokenKind::LessThanOrEquals,
                        TokenKind::GreaterThan,
                        TokenKind::GreaterThanOrEquals,
                        TokenKind::Equals,
                    ],
                    actual: operator.clone(),
                })
            }
        }
    }

    Ok((cursor, left))
}

enum RelationType {
    Equals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl RelationType {
    fn try_from_token(token: &Token) -> Option<Self> {
        match token.token_kind() {
            TokenKind::Equals => Some(RelationType::Equals),
            TokenKind::LessThan => Some(RelationType::LessThan),
            TokenKind::LessThanOrEquals => Some(RelationType::LessThanOrEquals),
            TokenKind::GreaterThan => Some(RelationType::GreaterThan),
            TokenKind::GreaterThanOrEquals => {
                Some(RelationType::GreaterThanOrEquals)
            }
            _ => None,
        }
    }
}

fn parse_primary<'a>(mut cursor: Cursor<'a>) -> ParseResult<'a, Node> {
    match cursor.next() {
        Some(token) => match token {
            // TODO: handle anonymous composite function calls: (f + g)(x)
            Token::Identifier(identifier_token) => {
                if let Some(Token::LeftParenthesis(..)) = cursor.peek() {
                    // function call
                    cursor.next(); // consume left parenthesis
                    let mut arguments = Vec::<Node>::new();
                    while let Ok((next_cursor, node)) = parse_expression(cursor) {
                        cursor = next_cursor;
                        arguments.push(node);
                        match cursor.next().ok_or_else(|| ParseError::UnexpectedEndOfInput)? {
                            Token::Separator(_) => (),
                            Token::RightParenthesis(_) => break,
                            token => return Err(ParseError::UnexpectedToken { 
                                expected: vec![
                                    TokenKind::RightParenthesis,
                                    TokenKind::Separator
                                ], 
                                actual: token.clone() 
                            })
                        }
                    }
                    if arguments.is_empty() {
                        // consume right parenthesis
                        match cursor.next() {
                            None => return Err(ParseError::UnexpectedEndOfInput),
                            Some(Token::RightParenthesis(_)) => (),
                            Some(token) => return Err(ParseError::UnexpectedToken { 
                                expected: vec![TokenKind::RightParenthesis], 
                                actual: token.clone() 
                            })
                        };
                    }
                    Ok((cursor, FunctionCall::new(Symbol::new(&identifier_token.name), arguments)))
                } else {
                    // symbol
                    Ok((cursor, Symbol::new(&identifier_token.name)))
                }
            }
            Token::Number(number_token) => {
                Ok((cursor, Number::new_node(number_token.value)))
            }
            Token::Boolean(boolean_token) => {
                Ok((cursor, Boolean::new(boolean_token.value)))
            }
            Token::LeftParenthesis(_) => {
                let (next_cursor, node) = parse_expression(cursor)?;
                cursor = next_cursor;
                match cursor.next().ok_or(ParseError::UnexpectedEndOfInput)? {
                    Token::RightParenthesis(_) => Ok((cursor, node)),
                    token => Err(ParseError::UnexpectedToken {
                        expected: vec![TokenKind::RightParenthesis],
                        actual: token.clone(),
                    }),
                }
            }
            Token::LeftBracket(_) => {
                let mut elements = Vec::<Node>::new();
                while let Ok((next_cursor, node)) = parse_expression(cursor) {
                    cursor = next_cursor;
                    elements.push(node);
                    match cursor.next().ok_or_else(|| ParseError::UnexpectedEndOfInput)? {
                        Token::Separator(_) => (),
                        Token::RightBracket(_) => break,
                        token => return Err(ParseError::UnexpectedToken { 
                            expected: vec![
                                TokenKind::RightBracket,
                                TokenKind::Separator
                            ], 
                            actual: token.clone() 
                        })
                    }
                }
                if elements.is_empty() {
                    // consume right bracket
                    match cursor.next() {
                        None => return Err(ParseError::UnexpectedEndOfInput),
                        Some(Token::RightBracket(_)) => (),
                        Some(token) => return Err(ParseError::UnexpectedToken { 
                            expected: vec![TokenKind::RightBracket], 
                            actual: token.clone() 
                        })
                    };
                }
                Ok((cursor, Tensor::new(elements)))
            }
            token => Err(ParseError::UnexpectedToken {
                expected: vec![
                    TokenKind::Identifier,
                    TokenKind::Boolean,
                    TokenKind::Number,
                    TokenKind::LeftParenthesis,
                    TokenKind::LeftBracket,
                ],
                actual: token.clone(),
            }),
        },
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Position, RightBracketToken, Span, tokenize};

    use super::*;

    #[test]
    fn sum_number_x2() {
        let tokens = tokenize("1 + 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![Number::new_node(1.0), Number::new_node(2.0)])
        );
    }

    #[test]
    fn sum_number_x3() {
        let tokens =
            tokenize("1 + 2 + 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0),
                Number::new_node(3.0)
            ])
        );
    }

    #[test]
    fn sum_number_negate_number() {
        let tokens = tokenize("1 - 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0),
                Negate::new(Number::new_node(2.0))
            ])
        );
    }
    #[test]
    fn sum_number_negate_number_number() {
        let tokens =
            tokenize("1 - 2 + 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0),
                Negate::new(Number::new_node(2.0)),
                Number::new_node(3.0)
            ])
        );
    }

    #[test]
    fn product_number_x2() {
        let tokens = tokenize("1 * 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![Number::new_node(1.0), Number::new_node(2.0)])
        );
    }

    #[test]
    fn product_number_x3() {
        let tokens =
            tokenize("1 * 2 * 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0),
                Number::new_node(3.0)
            ])
        );
    }

    #[test]
    fn sum_product() {
        let tokens =
            tokenize("1 + 2 * 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0),
                Product::new(vec![
                    Number::new_node(2.0),
                    Number::new_node(3.0)
                ])
            ])
        );
    }

    #[test]
    fn product_sum() {
        let tokens =
            tokenize("1 * 2 + 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Product::new(vec![
                    Number::new_node(1.0),
                    Number::new_node(2.0)
                ]),
                Number::new_node(3.0),
            ])
        );
    }

    #[test]
    fn division_number_x2() {
        let tokens = tokenize("1 / 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Division::new(Number::new_node(1.0), Number::new_node(2.0),)
        );
    }

    #[test]
    fn division_number_x3() {
        let tokens =
            tokenize("1 / 2 / 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Division::new(
                Division::new(Number::new_node(1.0), Number::new_node(2.0),),
                Number::new_node(3.0)
            )
        );
    }

    #[test]
    fn product_division() {
        let tokens =
            tokenize("1 * 2 / 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Number::new_node(1.0),
                Division::new(Number::new_node(2.0), Number::new_node(3.0),)
            ])
        );
    }

    #[test]
    fn product_sum_sum() {
        let tokens =
            tokenize("(1 + 2) * (3 + 4)", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Sum::new(vec![Number::new_node(1.0), Number::new_node(2.0),]),
                Sum::new(vec![Number::new_node(3.0), Number::new_node(4.0),]),
            ])
        );
    }

    #[test]
    fn power_number_x2() {
        let tokens =
            tokenize("1 ^ 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Power::new(Number::new_node(1.0), Number::new_node(2.0))
        );
    }

    #[test]
    fn power_number_x3() {
        let tokens =
            tokenize("1 ^ 2 ^ 3", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Power::new(
                Number::new_node(1.0),
                Power::new(Number::new_node(2.0), Number::new_node(3.0))
            )
        );
    }

    #[test]
    fn define_symbol() {
        let tokens = tokenize("a := 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Definition::new("a", Number::new_node(2.0))
        );
    }

    #[test]
    fn define_function() {
        let tokens = tokenize("f(x) := x", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Definition::new("f", Function::new(
                FunctionSignature::new()
                    .argument("x", |arg| arg.node_type(NodeType::Any)), 
                Symbol::new("x")
            ))
        );
    }

    #[test]
    fn equals() {
        let tokens = tokenize("a = 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Equals::new(Symbol::new("a"), Number::new_node(2.0))
        );
    }

    #[test]
    fn less_than() {
        let tokens = tokenize("a < 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            LessThan::new(Symbol::new("a"), Number::new_node(2.0))
        );
    }

    #[test]
    fn less_than_or_equals() {
        let tokens = tokenize("a <= 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            LessThanOrEquals::new(Symbol::new("a"), Number::new_node(2.0))
        );
    }

    #[test]
    fn greater_than() {
        let tokens = tokenize("a > 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            GreaterThan::new(Symbol::new("a"), Number::new_node(2.0))
        );
    }

    #[test]
    fn greater_than_or_equals() {
        let tokens = tokenize("a >= 2", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            GreaterThanOrEquals::new(Symbol::new("a"), Number::new_node(2.0))
        );
    }

    #[test]
    fn relation_chain_x2() {
        let tokens = tokenize("a < b < c", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            And::new(vec![
                LessThan::new(Symbol::new("a"), Symbol::new("b")),
                LessThan::new(Symbol::new("b"), Symbol::new("c")),
            ])
        );
    }

    #[test]
    fn relation_chain_x5() {
        let tokens = tokenize("a < b <= c = d >= e > f", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            And::new(vec![
                LessThan::new(Symbol::new("a"), Symbol::new("b")),
                LessThanOrEquals::new(Symbol::new("b"), Symbol::new("c")),
                Equals::new(Symbol::new("c"), Symbol::new("d")),
                GreaterThanOrEquals::new(Symbol::new("d"), Symbol::new("e")),
                GreaterThan::new(Symbol::new("e"), Symbol::new("f")),
            ])
        );
        assert!(cursor.is_eof())

    }

    #[test]
    fn function_call_no_args() {
        let tokens = tokenize("f()", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            FunctionCall::new(Symbol::new("f"), vec![])
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_single_arg() {
        let tokens = tokenize("f(x)", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            FunctionCall::new(Symbol::new("f"), vec![
                Symbol::new("x")
            ])
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_single_2_args() {
        let tokens = tokenize("f(x, y)", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            FunctionCall::new(Symbol::new("f"), vec![
                Symbol::new("x"),
                Symbol::new("y")
            ])
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_err_missing_right_parenthesis() {
        let tokens = tokenize("f(x, y", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let err = parse_expression(cursor).unwrap_err();
        assert_eq!(
            err,
            ParseError::UnexpectedEndOfInput
        );
    }
    
    #[test]
    fn function_call_err_unexpected_token() {
        let tokens = tokenize("f(x]", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let err = parse_expression(cursor).unwrap_err();
        assert_eq!(
            err,
            ParseError::UnexpectedToken { 
                expected: vec![
                    TokenKind::RightParenthesis,
                    TokenKind::Separator
                ], 
                actual: Token::RightBracket(
                    RightBracketToken { 
                        span: Span::new(
                            Position { byte_index: 3, char_index: 3 }, 
                            Position { byte_index: 3, char_index: 3 }
                        )
                    }
                ) 
            }
        );
    }

    #[test]
    fn sum_function_call_x2() {
        let tokens = tokenize("f(x) + f(y)", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Sum::new(vec![
                FunctionCall::new(Symbol::new("f"), vec![
                    Symbol::new("x"),
                ]),
                FunctionCall::new(Symbol::new("f"), vec![
                    Symbol::new("y"),
                ]),
            ])
            
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_no_elements() {
        let tokens = tokenize("[]", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![])
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_1_element() {
        let tokens = tokenize("[2]", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(2.0)
            ])
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_2_element() {
        let tokens = tokenize("[1, 2]", language::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new(vec![
                Number::new_node(1.0),
                Number::new_node(2.0)
            ])
        );
        assert!(cursor.is_eof())
    }
}

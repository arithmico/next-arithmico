use lexer::{
    ArrowToken, GetTokenKind, GetTokenSpan, IdentifierToken, RightBracketToken,
    RightParenthesisToken, SeparatorToken, Span, Token, TokenKind,
};
use node::{
    And, Boolean, Definition, Division, Equals, Factorial, Function,
    FunctionCall, FunctionSignature, GetNodeType, GreaterThan,
    GreaterThanOrEquals, LessThan, LessThanOrEquals, Negate, Node, NodeType,
    Number, Or, Power, Product, Sum, Symbol, Tensor,
};
use trace::{CombineHulls, Tracable, TracableMut};

use crate::{
    Error::{self, InvalidFunctionArgumentDeclaration, InvalidFunctionName},
    ParseResult,
    binding_power::GetBindingPower,
    cursor::Cursor,
};

pub(crate) fn parse_expression<'a>(
    cursor: Cursor<'a>,
) -> ParseResult<'a, Node> {
    parse_expression_pratt(cursor, 0)
}

fn parse_expression_pratt<'a>(
    mut cursor: Cursor<'a>,
    min_binding_power: u8,
) -> ParseResult<'a, Node> {
    let (mut cursor, mut left) = if let Some(Token::Minus(token)) =
        cursor.peek()
        && Some(min_binding_power) <= TokenKind::Minus.binding_power()
    {
        // consume minus token
        cursor.next();
        let (cursor, mut left) = parse_primary(cursor)?;
        let mut span = token.span;
        if let Some(s) = left.hull() {
            span = span.hull(&s);
        }
        left = Negate::new(left).with_span(span);
        (cursor, left)
    } else {
        parse_primary(cursor)?
    };

    while let Some(next_token) = cursor.peek() {
        if let (
            Some(TokenKind::RightParenthesis),
            Some(TokenKind::LeftParenthesis),
        ) = (cursor.current_token_kind(), cursor.peek_token_kind())
        {
            return Err(Error::MissingMultiplyBetween {
                span: Span::new(
                    // Safety: the match pattern garantees that this is always Some(_)
                    cursor.current_position().unwrap(),
                    // Safety: the match pattern garantees that this is always Some(_)
                    cursor.peek_position().unwrap(),
                ),
            });
        };

        let Some(binding_power) = next_token.binding_power() else {
            break;
        };

        if binding_power < min_binding_power {
            break;
        }

        let Some(operator) = cursor.next() else {
            break; // unreachable
        };

        if let Token::ExclamationMark(token) = operator {
            let mut span = token.get_span();
            if let Some(left_span) = left.hull() {
                span = span.hull(&left_span);
            }
            left = Factorial::new(left).with_span(span);
            continue;
        }

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
                    let span = (sum.trace(), &right).combine_hulls();
                    sum.elements.push(right);
                    left = left.with_optional_span(span);
                } else {
                    let span = (&left, &right).combine_hulls();
                    left = Sum::new(vec![left, right]).with_optional_span(span);
                }
            }
            TokenKind::Minus => {
                let mut span = operator.get_span();
                if let Some(right_span) = right.hull() {
                    span = span.hull(&right_span);
                }
                if let Node::Sum(sum) = &mut left {
                    sum.elements.push(Negate::new(right).with_span(span));
                    left = left.with_span(span).only_hull();
                } else {
                    let right = Negate::new(right).with_span(span);
                    let span = (&left, &right).combine_hulls();
                    left = Sum::new(vec![left, right]).with_optional_span(span);
                }
            }
            TokenKind::Asterisk => {
                if let Node::Product(product) = &mut left {
                    let span = (product.trace(), &right).combine_hulls();
                    product.elements.push(right);
                    left = left.with_optional_span(span);
                } else {
                    let span = (&left, &right).combine_hulls();
                    left = Product::new(vec![left, right])
                        .with_optional_span(span);
                }
            }
            TokenKind::Slash => {
                let span = (&left, &right).combine_hulls();
                left = Division::new(left, right).with_optional_span(span);
            }
            TokenKind::Caret => {
                let span = (&left, &right).combine_hulls();
                left = Power::new(left, right).with_optional_span(span);
            }
            TokenKind::Define => {
                let span = (&left, &right).combine_hulls();
                if let Node::Symbol(symbol) = left {
                    left = Definition::new(symbol.name, right)
                        .with_optional_span(span);
                } else if let Node::FunctionCall(function_call) = &left {
                    let mut signature = FunctionSignature::default();
                    for argument in &function_call.arguments {
                        if let Node::Symbol(symbol) = argument {
                            if signature.has_argument(&symbol.name) {
                                return Err(
                                    Error::DuplicateFunctionArgumentName {
                                        name: symbol.name.clone(),
                                    },
                                );
                            }
                            signature.add_argument(&symbol.name, |argument| {
                                argument.node_type(NodeType::Any)
                            });
                        } else {
                            // TODO: add span to error
                            return Err(InvalidFunctionArgumentDeclaration);
                        }
                    }
                    if let Node::Symbol(symbol) = function_call.target.as_ref()
                    {
                        left = Definition::new(
                            &symbol.name,
                            Function::new(signature, right)
                                .with_optional_span(span),
                        )
                        .with_optional_span(span)
                    } else {
                        return Err(InvalidFunctionName);
                    }
                } else {
                    // TODO: add span to error
                    return Err(Error::UnexpectedLeftSideOfDefinition {
                        node_type: left.node_type(),
                    });
                }
            }
            TokenKind::Or => {
                if let Node::Or(or) = &mut left {
                    let span = (or.trace(), &right).combine_hulls();
                    or.elements.push(right);
                    left = left.with_optional_span(span);
                } else {
                    let span = (&left, &right).combine_hulls();
                    left = Or::new(vec![left, right]).with_optional_span(span);
                }
            }
            TokenKind::And => {
                if let Node::And(and) = &mut left {
                    let span = (and.trace(), &right).combine_hulls();
                    and.elements.push(right);
                    left = left.with_optional_span(span);
                } else {
                    let span = (&left, &right).combine_hulls();
                    left = And::new(vec![left, right]).with_optional_span(span);
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
                    RelationType::try_from_token(operator).unwrap();
                items.push((relation_type, right));
                while let Some(token) = cursor.peek()
                    && let Some(relation_type) =
                        RelationType::try_from_token(token)
                {
                    cursor.next();
                    let (next_cursor, item) =
                        parse_expression_pratt(cursor, next_min_minding_power)?;
                    cursor = next_cursor;
                    items.push((relation_type, item));
                }
                let mut output = vec![];
                let mut outer_span = left.hull();
                let mut current_left = left;
                for (relation_type, item) in items {
                    match (outer_span, item.hull()) {
                        (None, Some(s)) => outer_span = Some(s),
                        (Some(s), None) => outer_span = Some(s),
                        (Some(left), Some(right)) => {
                            outer_span = Some(left.hull(&right))
                        }
                        _ => {}
                    };
                    let span = (&current_left, &item).combine_hulls();
                    match relation_type {
                        RelationType::Equals => {
                            output.push(
                                Equals::new(current_left, item.clone())
                                    .with_optional_span(span),
                            );
                            current_left = item;
                        }
                        RelationType::LessThan => {
                            output.push(
                                LessThan::new(current_left, item.clone())
                                    .with_optional_span(span),
                            );
                            current_left = item;
                        }
                        RelationType::LessThanOrEquals => {
                            output.push(
                                LessThanOrEquals::new(
                                    current_left,
                                    item.clone(),
                                )
                                .with_optional_span(span),
                            );
                            current_left = item;
                        }
                        RelationType::GreaterThan => {
                            output.push(
                                GreaterThan::new(current_left, item.clone())
                                    .with_optional_span(span),
                            );
                            current_left = item;
                        }
                        RelationType::GreaterThanOrEquals => {
                            output.push(
                                GreaterThanOrEquals::new(
                                    current_left,
                                    item.clone(),
                                )
                                .with_optional_span(span),
                            );
                            current_left = item;
                        }
                    }
                }

                if let Some(first) = output.first()
                    && output.len() == 1
                {
                    left = first.clone();
                } else {
                    left = And::new(output).with_optional_span(outer_span);
                }
            }
            _ => {
                return Err(Error::UnexpectedToken {
                    expected: vec![
                        TokenKind::Plus,
                        TokenKind::Minus,
                        TokenKind::Asterisk,
                        TokenKind::Slash,
                        TokenKind::Caret,
                        TokenKind::Define,
                        TokenKind::LessThan,
                        TokenKind::LessThanOrEquals,
                        TokenKind::GreaterThan,
                        TokenKind::GreaterThanOrEquals,
                        TokenKind::Equals,
                    ],
                    actual: operator.clone(),
                });
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

fn find_closing_parenthesis<'a>(mut cursor: Cursor<'a>) -> Option<Cursor<'a>> {
    let mut inner_left_parenthesis: usize = 0;
    while let Some(token) = cursor.next() {
        match token.token_kind() {
            TokenKind::LeftParenthesis => {
                inner_left_parenthesis += 1;
            }
            TokenKind::RightParenthesis => {
                if inner_left_parenthesis == 0 {
                    return Some(cursor);
                } else {
                    inner_left_parenthesis -= 1;
                }
            }
            _ => (),
        }
    }
    None
}

fn parse_primary<'a>(mut cursor: Cursor<'a>) -> ParseResult<'a, Node> {
    match cursor.next() {
        Some(token) => match token {
            // TODO: handle anonymous composite function calls: (f + g)(x)
            Token::Identifier(identifier_token) => {
                if let Some(Token::LeftParenthesis(..)) = cursor.peek() {
                    let mut outer_span = identifier_token.get_span();
                    // function call
                    cursor.next(); // consume left parenthesis
                    let mut arguments = Vec::<Node>::new();
                    while let Ok((next_cursor, node)) = parse_expression(cursor)
                    {
                        cursor = next_cursor;
                        arguments.push(node);

                        if let Token::RightParenthesis(token) = cursor
                            .expect_one_of(&[
                                TokenKind::RightParenthesis,
                                TokenKind::Separator,
                            ])?
                        {
                            outer_span = outer_span.hull(&token.get_span());
                            break;
                        }
                    }
                    if arguments.is_empty() {
                        // consume right parenthesis
                        outer_span = outer_span.hull(
                            &cursor
                                .expect::<RightParenthesisToken>()?
                                .get_span(),
                        );
                    }
                    Ok((
                        cursor,
                        FunctionCall::new(
                            Symbol::new(&identifier_token.name)
                                .with_span(identifier_token.get_span()),
                            arguments,
                        )
                        .with_span(outer_span),
                    ))
                } else {
                    // symbol
                    Ok((
                        cursor,
                        Symbol::new(&identifier_token.name)
                            .with_span(token.get_span()),
                    ))
                }
            }
            Token::Number(number_token) => Ok((
                cursor,
                Number::new_node(number_token.value)
                    .with_span(number_token.span),
            )),
            Token::Boolean(boolean_token) => Ok((
                cursor,
                Boolean::new(boolean_token.value).with_span(boolean_token.span),
            )),
            Token::LeftParenthesis(token) => {
                let Some(closing_cursor) = find_closing_parenthesis(cursor)
                else {
                    return Err(Error::MissingClosingParenthesis {
                        span: token.get_span(),
                    });
                };
                if closing_cursor.peek_token_kind() == Some(TokenKind::Arrow) {
                    let mut span = token.get_span();
                    // parse inline function declaration
                    let mut parameters = Vec::<String>::new();
                    // read function parameters e. g. (a, b, c)
                    while let Some(token) = cursor.next_if::<IdentifierToken>()
                    {
                        parameters.push(token.name.clone());
                        cursor.next_if::<SeparatorToken>();
                    }

                    // consume closing parenthesis
                    cursor.expect::<RightParenthesisToken>()?;

                    // consume arrow
                    cursor.expect::<ArrowToken>()?;

                    // parse function expression
                    let (next_cursor, node) = parse_expression_pratt(
                        cursor,
                        TokenKind::Define.binding_power().unwrap() + 1,
                    )?;
                    let mut signature = FunctionSignature::default();
                    for param in parameters {
                        if signature.has_argument(&param) {
                            return Err(Error::DuplicateFunctionArgumentName {
                                name: param,
                            });
                        }
                        signature.add_argument(param, |arg| {
                            arg.node_type(NodeType::Any)
                        });
                    }
                    let signature = signature.add_return_type(NodeType::Any);
                    if let Some(s) = node.hull() {
                        span = span.hull(&s);
                    }

                    Ok((
                        next_cursor,
                        Function::new(signature, node).with_span(span),
                    ))
                } else {
                    // parse expression between parenthesis
                    let (next_cursor, node) = parse_expression(cursor)?;
                    cursor = next_cursor;
                    match cursor.next().ok_or(Error::UnexpectedEndOfInput)? {
                        Token::RightParenthesis(_) => Ok((cursor, node)),
                        token => Err(Error::UnexpectedToken {
                            expected: vec![TokenKind::RightParenthesis],
                            actual: token.clone(),
                        }),
                    }
                }
            }
            Token::LeftBracket(token) => {
                let mut span = token.span;
                let mut elements = Vec::<Node>::new();
                while let Ok((next_cursor, node)) = parse_expression(cursor) {
                    cursor = next_cursor;
                    elements.push(node);
                    match cursor.next().ok_or(Error::UnexpectedEndOfInput)? {
                        Token::Separator(_) => (),
                        Token::RightBracket(token) => {
                            span = span.hull(&token.span);
                            break;
                        }
                        token => {
                            return Err(Error::UnexpectedToken {
                                expected: vec![
                                    TokenKind::RightBracket,
                                    TokenKind::Separator,
                                ],
                                actual: token.clone(),
                            });
                        }
                    }
                }
                if elements.is_empty() {
                    // consume right bracket
                    span = span.hull(
                        &cursor.expect::<RightBracketToken>()?.get_span(),
                    );
                }
                Ok((cursor, Tensor::new_node(elements).with_span(span)))
            }
            token => Err(Error::UnexpectedToken {
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
        None => Err(Error::UnexpectedEndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Position, RightBracketToken, Span, tokenize};

    use super::*;

    #[test]
    fn symbol() {
        let tokens = tokenize("foo", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Symbol::new("foo").with_span(Span::new_between(0, 2))
        );
    }

    #[test]
    fn number() {
        let tokens = tokenize("1", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Number::new_node(1.0)
                .with_span(Span::new(Position::new(0, 0), Position::new(0, 0)))
        );
    }

    #[test]
    fn boolean_true() {
        let tokens = tokenize("true", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Boolean::new(true)
                .with_span(Span::new(Position::new(0, 0), Position::new(3, 3)))
        );
    }

    #[test]
    fn boolean_false() {
        let tokens = tokenize("false", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Boolean::new(false).with_span(Span::new_between(0, 4))
        );
    }

    #[test]
    fn sum_number_x2() {
        let tokens = tokenize("1 + 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            ])
            .with_span(Span::new_between(0, 4))
        );
    }

    #[test]
    fn sum_number_x3() {
        let tokens = tokenize("1 + 2 + 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                Number::new_node(3.0).with_span(Span::new_between(8, 8)),
            ])
            .with_span(Span::new_between(0, 8))
        );
    }

    #[test]
    fn single_negate() {
        let tokens = tokenize("-3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Negate::new(
                Number::new_node(3.0).with_span(Span::new_between(1, 1))
            )
            .with_span(Span::new_between(0, 1))
        );
    }

    #[test]
    fn sum_number_negate_number() {
        let tokens = tokenize("1 - 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Sum::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Negate::new(
                    Number::new_node(2.0).with_span(Span::new_between(4, 4))
                )
                .with_span(Span::new_between(2, 4))
            ])
            .with_span(Span::new_between(0, 4))
        );
    }
    #[test]
    fn sum_number_number_negate_number() {
        let tokens = tokenize("1 - 2 + 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Negate::new(
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                )
                .with_span(Span::new_between(2, 4)),
                Number::new_node(3.0).with_span(Span::new_between(8, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn product_number_x2() {
        let tokens = tokenize("1 * 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            ])
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn product_number_x3() {
        let tokens = tokenize("1 * 2 * 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                Number::new_node(3.0).with_span(Span::new_between(8, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn sum_product() {
        let tokens = tokenize("1 + 2 * 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Product::new(vec![
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                    Number::new_node(3.0).with_span(Span::new_between(8, 8)),
                ])
                .with_span(Span::new_between(4, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn product_sum() {
        let tokens = tokenize("1 * 2 + 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Sum::new(vec![
                Product::new(vec![
                    Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                ])
                .with_span(Span::new_between(0, 4)),
                Number::new_node(3.0).with_span(Span::new_between(8, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn division_number_x2() {
        let tokens = tokenize("1 / 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Division::new(
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            )
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn division_number_x3() {
        let tokens = tokenize("1 / 2 / 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Division::new(
                Division::new(
                    Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                )
                .with_span(Span::new_between(0, 4)),
                Number::new_node(3.0).with_span(Span::new_between(8, 8)),
            )
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn product_division() {
        let tokens = tokenize("1 * 2 / 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Product::new(vec![
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Division::new(
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                    Number::new_node(3.0).with_span(Span::new_between(8, 8)),
                )
                .with_span(Span::new_between(4, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn product_sum_sum() {
        let tokens =
            tokenize("(1 + 2) * (3 + 4)", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Product::new(vec![
                Sum::new(vec![
                    Number::new_node(1.0).with_span(Span::new_between(1, 1)),
                    Number::new_node(2.0).with_span(Span::new_between(5, 5)),
                ])
                .with_span(Span::new_between(1, 5)),
                Sum::new(vec![
                    Number::new_node(3.0).with_span(Span::new_between(11, 11)),
                    Number::new_node(4.0).with_span(Span::new_between(15, 15)),
                ])
                .with_span(Span::new_between(11, 15)),
            ])
            .with_span(Span::new_between(1, 15)),
        );
    }

    #[test]
    fn power_number_x2() {
        let tokens = tokenize("1 ^ 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Power::new(
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            )
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn power_number_x3() {
        let tokens = tokenize("1 ^ 2 ^ 3", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Power::new(
                Number::new_node(1.0).with_span(Span::new_between(0, 0)),
                Power::new(
                    Number::new_node(2.0).with_span(Span::new_between(4, 4)),
                    Number::new_node(3.0).with_span(Span::new_between(8, 8)),
                )
                .with_span(Span::new_between(4, 8)),
            )
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn define_symbol() {
        let tokens = tokenize("a := 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        dbg!(&output);
        assert_eq!(
            output,
            Definition::new(
                "a",
                Number::new_node(2.0).with_span(Span::new_between(5, 5))
            )
            .with_span(Span::new_between(0, 5))
        );
    }

    #[test]
    fn function_call() {
        let tokens = tokenize("f(x)", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            FunctionCall::new(
                Symbol::new("f").with_span(Span::new_between(0, 0)),
                vec![Symbol::new("x").with_span(Span::new_between(2, 2))]
            )
            .with_span(Span::new_between(0, 3))
        );
    }

    #[test]
    fn define_function() {
        let tokens = tokenize("f(x) := x", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Definition::new(
                "f",
                Function::new(
                    FunctionSignature::default()
                        .argument("x", |arg| arg.node_type(NodeType::Any)),
                    Symbol::new("x").with_span(Span::new_between(8, 8))
                )
                .with_span(Span::new_between(0, 8))
            )
            .with_span(Span::new_between(0, 8))
        );
    }

    #[test]
    fn equals() {
        let tokens = tokenize("a = 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            Equals::new(
                Symbol::new("a").with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            )
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn less_than() {
        let tokens = tokenize("a < 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            LessThan::new(
                Symbol::new("a").with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4))
            )
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn less_than_or_equals() {
        let tokens = tokenize("a <= 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            LessThanOrEquals::new(
                Symbol::new("a").with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(5, 5)),
            )
            .with_span(Span::new_between(0, 5)),
        );
    }

    #[test]
    fn greater_than() {
        let tokens = tokenize("a > 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            GreaterThan::new(
                Symbol::new("a").with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4)),
            )
            .with_span(Span::new_between(0, 4)),
        );
    }

    #[test]
    fn greater_than_or_equals() {
        let tokens = tokenize("a >= 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        assert_eq!(
            parse_expression(cursor).unwrap().1,
            GreaterThanOrEquals::new(
                Symbol::new("a").with_span(Span::new_between(0, 0)),
                Number::new_node(2.0).with_span(Span::new_between(5, 5)),
            )
            .with_span(Span::new_between(0, 5)),
        );
    }

    #[test]
    fn relation_chain_x2() {
        let tokens = tokenize("a < b < c", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            And::new(vec![
                LessThan::new(
                    Symbol::new("a").with_span(Span::new_between(0, 0)),
                    Symbol::new("b").with_span(Span::new_between(4, 4)),
                )
                .with_span(Span::new_between(0, 4)),
                LessThan::new(
                    Symbol::new("b").with_span(Span::new_between(4, 4)),
                    Symbol::new("c").with_span(Span::new_between(8, 8)),
                )
                .with_span(Span::new_between(4, 8)),
            ])
            .with_span(Span::new_between(0, 8)),
        );
    }

    #[test]
    fn relation_chain_x5() {
        let tokens =
            tokenize("a < b <= c = d >= e > f", common::Language::English)
                .unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, output) = parse_expression(cursor).unwrap();
        assert_eq!(
            output,
            And::new(vec![
                LessThan::new(
                    Symbol::new("a").with_span(Span::new_between(0, 0)),
                    Symbol::new("b").with_span(Span::new_between(4, 4)),
                )
                .with_span(Span::new_between(0, 4)),
                LessThanOrEquals::new(
                    Symbol::new("b").with_span(Span::new_between(4, 4)),
                    Symbol::new("c").with_span(Span::new_between(9, 9)),
                )
                .with_span(Span::new_between(4, 9)),
                Equals::new(
                    Symbol::new("c").with_span(Span::new_between(9, 9)),
                    Symbol::new("d").with_span(Span::new_between(13, 13)),
                )
                .with_span(Span::new_between(9, 13)),
                GreaterThanOrEquals::new(
                    Symbol::new("d").with_span(Span::new_between(13, 13)),
                    Symbol::new("e").with_span(Span::new_between(18, 18)),
                )
                .with_span(Span::new_between(13, 18)),
                GreaterThan::new(
                    Symbol::new("e").with_span(Span::new_between(18, 18)),
                    Symbol::new("f").with_span(Span::new_between(22, 22)),
                )
                .with_span(Span::new_between(18, 22)),
            ])
            .with_span(Span::new_between(0, 22)),
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_no_args() {
        let tokens = tokenize("f()", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, output) = parse_expression(cursor).unwrap();
        assert_eq!(
            output,
            FunctionCall::new(
                Symbol::new("f").with_span(Span::new_between(0, 0)),
                vec![]
            )
            .with_span(Span::new_between(0, 2))
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_single_arg() {
        let tokens = tokenize("f(x)", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            FunctionCall::new(
                Symbol::new("f").with_span(Span::new_between(0, 0)),
                vec![Symbol::new("x").with_span(Span::new_between(2, 2)),]
            )
            .with_span(Span::new_between(0, 3)),
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_single_2_args() {
        let tokens = tokenize("f(x, y)", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            FunctionCall::new(
                Symbol::new("f").with_span(Span::new_between(0, 0)),
                vec![
                    Symbol::new("x").with_span(Span::new_between(2, 2)),
                    Symbol::new("y").with_span(Span::new_between(5, 5)),
                ]
            )
            .with_span(Span::new_between(0, 6)),
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn function_call_err_missing_right_parenthesis() {
        let tokens = tokenize("f(x, y", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let err = parse_expression(cursor).unwrap_err();
        assert_eq!(err, Error::UnexpectedEndOfInput);
    }

    #[test]
    fn function_call_err_unexpected_token() {
        let tokens = tokenize("f(x]", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let err = parse_expression(cursor).unwrap_err();
        assert_eq!(
            err,
            Error::UnexpectedToken {
                expected: vec![
                    TokenKind::RightParenthesis,
                    TokenKind::Separator
                ],
                actual: Token::RightBracket(RightBracketToken {
                    span: Span::new(
                        Position {
                            byte_index: 3,
                            char_index: 3
                        },
                        Position {
                            byte_index: 3,
                            char_index: 3
                        }
                    )
                })
            }
        );
    }

    #[test]
    fn sum_function_call_x2() {
        let tokens =
            tokenize("f(x) + f(y)", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Sum::new(vec![
                FunctionCall::new(
                    Symbol::new("f").with_span(Span::new_between(0, 0)),
                    vec![Symbol::new("x").with_span(Span::new_between(2, 2)),]
                )
                .with_span(Span::new_between(0, 3)),
                FunctionCall::new(
                    Symbol::new("f").with_span(Span::new_between(7, 7)),
                    vec![Symbol::new("y").with_span(Span::new_between(9, 9))]
                )
                .with_span(Span::new_between(7, 10)),
            ])
            .with_span(Span::new_between(0, 10)),
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_no_elements() {
        let tokens = tokenize("[]", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![]).with_span(Span::new_between(0, 1))
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_1_element() {
        let tokens = tokenize("[2]", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![
                Number::new_node(2.0).with_span(Span::new_between(1, 1))
            ])
            .with_span(Span::new_between(0, 2))
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn tensor_2_element() {
        let tokens = tokenize("[1, 2]", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let (cursor, result) = parse_expression(cursor).unwrap();
        assert_eq!(
            result,
            Tensor::new_node(vec![
                Number::new_node(1.0).with_span(Span::new_between(1, 1)),
                Number::new_node(2.0).with_span(Span::new_between(4, 4))
            ])
            .with_span(Span::new_between(0, 5))
        );
        assert!(cursor.is_eof())
    }

    #[test]
    fn or_boolean_x2() {
        let tokens =
            tokenize("true | false", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Or::new(vec![
                Boolean::new(true).with_span(Span::new_between(0, 3)),
                Boolean::new(false).with_span(Span::new_between(7, 11)),
            ])
            .with_span(Span::new_between(0, 11)),
        );
    }

    #[test]
    fn or_boolean_x3() {
        let tokens =
            tokenize("true | false | true", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Or::new(vec![
                Boolean::new(true).with_span(Span::new_between(0, 3)),
                Boolean::new(false).with_span(Span::new_between(7, 11)),
                Boolean::new(true).with_span(Span::new_between(15, 18)),
            ])
            .with_span(Span::new_between(0, 18)),
        );
    }

    #[test]
    fn and_boolean_x2() {
        let tokens =
            tokenize("true & false", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            And::new(vec![
                Boolean::new(true).with_span(Span::new_between(0, 3)),
                Boolean::new(false).with_span(Span::new_between(7, 11)),
            ])
            .with_span(Span::new_between(0, 11)),
        );
    }

    #[test]
    fn and_boolean_x3() {
        let tokens =
            tokenize("true & false & true", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            And::new(vec![
                Boolean::new(true).with_span(Span::new_between(0, 3)),
                Boolean::new(false).with_span(Span::new_between(7, 11)),
                Boolean::new(true).with_span(Span::new_between(15, 18)),
            ])
            .with_span(Span::new_between(0, 18)),
        );
    }

    #[test]
    fn or_and() {
        let tokens =
            tokenize("true | false & true", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Or::new(vec![
                Boolean::new(true).with_span(Span::new_between(0, 3)),
                And::new(vec![
                    Boolean::new(false).with_span(Span::new_between(7, 11)),
                    Boolean::new(true).with_span(Span::new_between(15, 18)),
                ])
                .with_span(Span::new_between(7, 18)),
            ])
            .with_span(Span::new_between(0, 18)),
        );
    }

    #[test]
    fn and_or() {
        let tokens =
            tokenize("true & false | true", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Or::new(vec![
                And::new(vec![
                    Boolean::new(true).with_span(Span::new_between(0, 3)),
                    Boolean::new(false).with_span(Span::new_between(7, 11)),
                ])
                .with_span(Span::new_between(0, 11)),
                Boolean::new(true).with_span(Span::new_between(15, 18)),
            ])
            .with_span(Span::new_between(0, 18)),
        );
    }

    #[test]
    fn factorial() {
        let tokens = tokenize("10!", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Factorial::new(
                Number::new_node(10.0).with_span(Span::new_between(0, 1))
            )
            .with_span(Span::new_between(0, 2)),
        );
    }

    #[test]
    fn function_no_arg() {
        let tokens = tokenize("() -> 2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Function::new(
                FunctionSignature::default().add_return_type(NodeType::Any),
                Number::new_node(2.0).with_span(Span::new_between(6, 6))
            )
            .with_span(Span::new_between(0, 6)),
        );
    }

    #[test]
    fn function_1_arg() {
        let tokens = tokenize("(x) -> x^2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Function::new(
                FunctionSignature::default()
                    .argument("x", |arg| arg.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Power::new(
                    Symbol::new("x").with_span(Span::new_between(7, 7)),
                    Number::new_node(2.0).with_span(Span::new_between(9, 9))
                )
                .with_span(Span::new_between(7, 9))
            )
            .with_span(Span::new_between(0, 9)),
        );
    }

    #[test]
    fn function_2_args() {
        let tokens =
            tokenize("(x, y) -> x^2", common::Language::English).unwrap();
        let cursor = Cursor::new(&tokens);
        let output = parse_expression(cursor).unwrap().1;
        assert_eq!(
            output,
            Function::new(
                FunctionSignature::default()
                    .argument("x", |arg| arg.node_type(NodeType::Any))
                    .argument("y", |arg| arg.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Power::new(
                    Symbol::new("x").with_span(Span::new_between(10, 10)),
                    Number::new_node(2.0).with_span(Span::new_between(12, 12))
                )
                .with_span(Span::new_between(10, 12))
            )
            .with_span(Span::new_between(0, 12)),
        );
    }
}

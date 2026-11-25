use lexer::{
    ArrowToken, GetTokenSpan, IdentifierToken, LeftParenthesisToken,
    RightBraceToken, RightBracketToken, RightParenthesisToken, SeparatorToken,
    Span, Token, TokenKind,
};
use node::{
    Boolean, Division, Function, FunctionCall, FunctionSignature, Node,
    NodeType, Number, Symbol, Tensor,
};
use trace::{Tracable, TracableMut};

use crate::{
    Error, ParseResult,
    binding_power::GetBindingPower,
    cursor::Cursor,
    expression::{
        group::{
            find_closing_brace, find_closing_bracket, find_closing_parenthesis,
        },
        parse_expression, parse_expression_pratt,
    },
};

pub fn parse_primary<'a>(mut cursor: Cursor<'a>) -> ParseResult<'a, Node> {
    let token = cursor.next_or_err()?;

    match token {
        // TODO: handle anonymous composite function calls: (f + g)(x)
        Token::Identifier(identifier_token) => {
            if let Some(Token::LeftParenthesis(..)) = cursor.peek() {
                // function call
                let mut outer_span = identifier_token.get_span();
                let left_parenthesis =
                    cursor.expect::<LeftParenthesisToken>()?;
                find_closing_parenthesis(cursor).ok_or(
                    Error::MissingClosingParenthesis {
                        span: left_parenthesis.get_span(),
                    },
                )?;

                let mut arguments = Vec::<Node>::new();
                while let Ok((next_cursor, node)) = parse_expression(cursor) {
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
                        &cursor.expect::<RightParenthesisToken>()?.get_span(),
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
            Number::new_node(number_token.value).with_span(number_token.span),
        )),
        Token::Boolean(boolean_token) => Ok((
            cursor,
            Boolean::new(boolean_token.value).with_span(boolean_token.span),
        )),
        Token::LeftParenthesis(token) => {
            let Some(closing_cursor) = find_closing_parenthesis(cursor) else {
                return Err(Error::MissingClosingParenthesis {
                    span: token.get_span(),
                });
            };
            if closing_cursor.peek_token_kind() == Some(TokenKind::Arrow) {
                let mut span = token.get_span();
                // parse inline function declaration
                let mut parameters = Vec::<(String, Span)>::new();
                // read function parameters e. g. (a, b, c)
                while let Some(token) = cursor.next_if::<IdentifierToken>() {
                    parameters.push((token.name.clone(), token.get_span()));
                    cursor.next_if::<SeparatorToken>();
                }

                // consume closing parenthesis
                cursor.expect::<RightParenthesisToken>()?;

                // consume arrow
                cursor.expect::<ArrowToken>()?;

                // parse function expression
                let (next_cursor, node) = parse_expression_pratt(
                    cursor,
                    TokenKind::Define.binding_power().unwrap_or_default() + 1,
                )?;
                let mut signature = FunctionSignature::default();
                for (param, span) in parameters {
                    if signature.has_argument(&param) {
                        return Err(Error::DuplicateFunctionArgumentName {
                            name: param,
                            span,
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
                match cursor.next_or_err()? {
                    Token::RightParenthesis(_) => Ok((cursor, node)),
                    token => Err(Error::UnexpectedToken {
                        expected: vec![TokenKind::RightParenthesis],
                        actual: token.clone(),
                    }),
                }
            }
        }
        Token::LeftBracket(token) => {
            let mut elements = Vec::<Node>::new();
            // check if closing bracket exists
            find_closing_bracket(cursor).ok_or(
                Error::MissingClosingBracket {
                    span: token.get_span(),
                },
            )?;

            while cursor.peek_token_kind_or_err()? != TokenKind::RightBracket {
                let (next_cursor, node) = parse_expression(cursor)?;
                cursor = next_cursor;
                elements.push(node);
                if let Token::Separator(_) = cursor.peek_or_err()? {
                    cursor.next_or_err()?;
                }
            }

            let right_bracket_span =
                cursor.expect::<RightBracketToken>()?.get_span();
            let span = token.span.hull(&right_bracket_span);
            Ok((cursor, Tensor::new_node(elements).with_span(span)))
        }
        Token::LeftBrace(token) if cfg!(feature = "latex_braces") => {
            // check if closing brace exists
            find_closing_brace(cursor).ok_or(Error::MissingClosingBrace {
                span: token.get_span(),
            })?;

            // parse expression between braces
            let (next_cursor, node) = parse_expression(cursor)?;
            cursor = next_cursor;
            cursor.expect::<RightBraceToken>()?;
            Ok((cursor, node))
        }
        Token::Backslash(token) if cfg!(feature = "latex_backslash") => {
            let command = cursor.expect::<IdentifierToken>()?;
            let mut hull = token.get_span();
            match command.name.as_str() {
                "sqrt" if cfg!(feature = "latex_sqrt") => {
                    let (cursor, argument) = parse_primary(cursor)?;
                    if let Some(span) = argument.hull() {
                        hull = hull.hull(&span);
                    }
                    Ok((
                        cursor,
                        FunctionCall::new(Symbol::new("sqrt"), vec![argument])
                            .with_span(hull),
                    ))
                }
                "frac" if cfg!(feature = "latex_sqrt") => {
                    let (cursor, dividend) = parse_primary(cursor)?;
                    let (cursor, divisor) = parse_primary(cursor)?;
                    if let Some(span) = divisor.hull() {
                        hull = hull.hull(&span);
                    }
                    Ok((
                        cursor,
                        Division::new(dividend, divisor).with_span(hull),
                    ))
                }
                name => Err(Error::UnsupportedLatexCommand {
                    span: command.get_span(),
                    name: name.to_string(),
                }),
            }
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
    }
}

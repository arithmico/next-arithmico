use lexer::{GetTokenKind, TokenKind};

use crate::cursor::Cursor;

pub fn find_closing_parenthesis<'a>(
    mut cursor: Cursor<'a>,
) -> Option<Cursor<'a>> {
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

pub fn find_closing_brace<'a>(mut cursor: Cursor<'a>) -> Option<Cursor<'a>> {
    let mut inner_left_parenthesis: usize = 0;
    while let Some(token) = cursor.next() {
        match token.token_kind() {
            TokenKind::LeftBrace => {
                inner_left_parenthesis += 1;
            }
            TokenKind::RightBrace => {
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

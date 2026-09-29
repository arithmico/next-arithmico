use std::{iter::Enumerate, ops::RangeInclusive, str::CharIndices};

use itertools::{PeekNth, peek_nth};

use crate::{Position, Span};

pub struct LexerCursor<'a> {
    iter: PeekNth<Enumerate<CharIndices<'a>>>,
}

impl<'a> Iterator for LexerCursor<'a> {
    type Item = (Position, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (char_index, (byte_index, char)) = self.iter.next()?;

        Some((
            Position {
                byte_index,
                char_index,
            },
            char,
        ))
    }
}

impl<'a> LexerCursor<'a> {
    pub fn from(input: &'a str) -> Self {
        Self {
            iter: peek_nth(input.char_indices().enumerate()),
        }
    }

    fn peek_nth(&mut self, index: usize) -> Option<(Position, char)> {
        let (char_index, (byte_index, char)) =
            self.iter.peek_nth(index).copied()?;
        Some((
            Position {
                byte_index,
                char_index,
            },
            char,
        ))
    }

    pub fn matches(&mut self, sequence: &str) -> Option<Span> {
        let (from, _) = self.peek_nth(0)?;
        let mut to = from;
        if sequence.chars().enumerate().all(|(index, s_char)| {
            match self.peek_nth(index) {
                Some((pos, char)) => {
                    to = pos;
                    char == s_char
                }
                None => false,
            }
        }) {
            Some(Span::new(from, to))
        } else {
            None
        }
    }

    pub fn match_one_of(
        &mut self,
        ranges: &[RangeInclusive<char>],
    ) -> Option<Position> {
        if let Some((position, front_char)) = self.peek_nth(0)
            && ranges.iter().any(|range| range.contains(&front_char))
        {
            return Some(position);
        }
        None
    }

    pub fn match_one_of_and_advance(
        &mut self,
        ranges: &[RangeInclusive<char>],
    ) -> Option<Position> {
        let position = self.match_one_of(ranges);
        if position.is_some() {
            self.iter.next();
        }
        position
    }

    pub fn match_many_of_and_advance(
        &mut self,
        ranges: &[RangeInclusive<char>],
    ) -> Option<Span> {
        let from = self.match_one_of_and_advance(ranges)?;
        let mut to = from;

        while let Some(position) = self.match_one_of_and_advance(ranges) {
            to = position;
        }

        Some(Span::new(from, to))
    }

    pub fn match_and_advance(&mut self, sequence: &str) -> Option<Span> {
        if let Some(span) = self.matches(sequence) {
            sequence.chars().for_each(|_| {
                self.next();
            });
            Some(span)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_next() {
        let mut cursor = LexerCursor::from("abc");

        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'a'
            ))
        );
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 1,
                    char_index: 1
                },
                'b'
            ))
        );
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 2,
                    char_index: 2
                },
                'c'
            ))
        );
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn matches_success() {
        let mut cursor = LexerCursor::from("fn main() {}");

        // Should match successfully
        assert!(cursor.matches("fn ").is_some());

        // `matches` should NOT advance the iterator
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'f'
            ))
        );
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 1,
                    char_index: 1
                },
                'n'
            ))
        );
    }

    #[test]
    fn matches_failure() {
        let mut cursor = LexerCursor::from("let x = 10;");

        // Fails because the sequence doesn't match
        assert!(!cursor.matches("const").is_some());

        // Fails because the requested sequence is longer than the remaining input
        assert!(!cursor.matches("let x = 10; ").is_some());

        // Ensure iterator didn't advance on failure
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'l'
            ))
        );
    }

    #[test]
    fn match_and_advance_success() {
        let mut cursor = LexerCursor::from("return true;");

        // Should match and consume "return "
        assert!(cursor.match_and_advance("return ").is_some());

        // The next character should be 't' at index 7
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 7,
                    char_index: 7
                },
                't'
            ))
        );
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 8,
                    char_index: 8
                },
                'r'
            ))
        );
    }

    #[test]
    fn match_and_advance_failure() {
        let mut cursor = LexerCursor::from("if (x > 5)");

        // Fails to match
        assert!(!cursor.match_and_advance("while").is_some());

        // Iterator should remain entirely untouched
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'i'
            ))
        );
    }

    #[test]
    fn match_and_advance_partial_failure() {
        let mut cursor = LexerCursor::from("function");

        // Matches the first 4 characters but fails on the 5th
        assert!(!cursor.match_and_advance("func_").is_some());

        // The iterator must NOT be partially advanced
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'f'
            ))
        );
    }

    #[test]
    fn match_one_of_success() {
        let mut cursor = LexerCursor::from("hello");
        let ranges = ['a'..='z'];

        // Should successfully match the 'h'
        assert_eq!(
            cursor.match_one_of(&ranges),
            Some(Position {
                byte_index: 0,
                char_index: 0
            })
        );

        // `match_one_of` should NOT advance the iterator
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'h'
            ))
        );
    }

    #[test]
    fn match_one_of_failure() {
        let mut cursor = LexerCursor::from("123");
        let ranges = ['a'..='z'];

        // Fails because '1' is not in the range 'a'..='z'
        assert_eq!(cursor.match_one_of(&ranges), None);

        // Ensure iterator didn't advance
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                '1'
            ))
        );
    }

    #[test]
    fn match_one_of_and_advance_success() {
        let mut cursor = LexerCursor::from("123");
        let ranges = ['0'..='9'];

        // Should match and consume '1'
        assert_eq!(
            cursor.match_one_of_and_advance(&ranges),
            Some(Position {
                byte_index: 0,
                char_index: 0
            })
        );

        // The next character should be '2' at index 1
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 1,
                    char_index: 1
                },
                '2'
            ))
        );
    }

    #[test]
    fn match_one_of_and_advance_failure() {
        let mut cursor = LexerCursor::from("abc");
        let ranges = ['0'..='9'];

        // Fails to match
        assert_eq!(cursor.match_one_of_and_advance(&ranges), None);

        // Iterator should remain entirely untouched
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'a'
            ))
        );
    }

    #[test]
    fn match_many_of_and_advance_success() {
        let mut cursor = LexerCursor::from("123abc456");
        let ranges = ['0'..='9'];

        // Should match and consume "123"
        let span = cursor.match_many_of_and_advance(&ranges);
        assert_eq!(
            span,
            Some(Span::new(
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                Position {
                    byte_index: 2,
                    char_index: 2
                }
            ))
        );

        // The next character should be 'a' at index 3
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 3,
                    char_index: 3
                },
                'a'
            ))
        );
    }

    #[test]
    fn match_many_of_and_advance_failure() {
        let mut cursor = LexerCursor::from("abc123");
        let ranges = ['0'..='9'];

        // Fails because the first character 'a' does not match
        assert_eq!(cursor.match_many_of_and_advance(&ranges), None);

        // Iterator should remain entirely untouched
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                'a'
            ))
        );
    }

    #[test]
    fn match_one_of_multibyte_char() {
        // '🚀' is a 4-byte character
        let mut cursor = LexerCursor::from("🚀a");

        // Advance past the multi-byte character
        assert_eq!(
            cursor.next(),
            Some((
                Position {
                    byte_index: 0,
                    char_index: 0
                },
                '🚀'
            ))
        );

        let ranges = ['a'..='z'];

        // The next character 'a' has a char_index of 1, but a byte_index of 4
        assert_eq!(
            cursor.match_one_of(&ranges),
            Some(Position {
                byte_index: 4,
                char_index: 1
            })
        );
    }
}

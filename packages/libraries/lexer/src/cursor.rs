use std::{iter::Enumerate, str::Chars};

use itertools::{peek_nth, PeekNth};

pub struct LexerCursor<'a> {
    iter: PeekNth<Enumerate<Chars<'a>>>,
}

impl<'a> Iterator for LexerCursor<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> LexerCursor<'a> {
    pub fn from(input: &'a str) -> Self {
        Self {
            iter: peek_nth(input.chars().enumerate()),
        }
    }

    pub fn matches(&mut self, sequence: &str) -> bool {
        sequence.chars().enumerate().all(|(index, s_char)| {
            matches!(
                self.iter.peek_nth(index),
                Some((_, char)) if s_char == *char
            )
        })
    }

    pub fn match_and_advance(&mut self, sequence: &str) -> bool {
        if self.matches(sequence) {
            sequence.chars().for_each(|_| {
                self.next();
            });
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_next() {
        let mut cursor = LexerCursor::from("abc");

        assert_eq!(cursor.next(), Some((0, 'a')));
        assert_eq!(cursor.next(), Some((1, 'b')));
        assert_eq!(cursor.next(), Some((2, 'c')));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn matches_success() {
        let mut cursor = LexerCursor::from("fn main() {}");

        // Should match successfully
        assert!(cursor.matches("fn "));

        // `matches` should NOT advance the iterator
        assert_eq!(cursor.next(), Some((0, 'f')));
        assert_eq!(cursor.next(), Some((1, 'n')));
    }

    #[test]
    fn matches_failure() {
        let mut cursor = LexerCursor::from("let x = 10;");

        // Fails because the sequence doesn't match
        assert!(!cursor.matches("const"));

        // Fails because the requested sequence is longer than the remaining input
        assert!(!cursor.matches("let x = 10; "));

        // Ensure iterator didn't advance on failure
        assert_eq!(cursor.next(), Some((0, 'l')));
    }

    #[test]
    fn match_and_advance_success() {
        let mut cursor = LexerCursor::from("return true;");

        // Should match and consume "return "
        assert!(cursor.match_and_advance("return "));

        // The next character should be 't' at index 7
        assert_eq!(cursor.next(), Some((7, 't')));
        assert_eq!(cursor.next(), Some((8, 'r')));
    }

    #[test]
    fn match_and_advance_failure() {
        let mut cursor = LexerCursor::from("if (x > 5)");

        // Fails to match
        assert!(!cursor.match_and_advance("while"));

        // Iterator should remain entirely untouched
        assert_eq!(cursor.next(), Some((0, 'i')));
    }

    #[test]
    fn match_and_advance_partial_failure() {
        let mut cursor = LexerCursor::from("function");

        // Matches the first 4 characters but fails on the 5th
        assert!(!cursor.match_and_advance("func_"));

        // The iterator must NOT be partially advanced
        assert_eq!(cursor.next(), Some((0, 'f')));
    }
}

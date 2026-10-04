use lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    spans: Vec<Span>,
}

impl Frame {
    pub fn new(span: Span) -> Self {
        Self { spans: vec![span] }
    }

    pub fn push(&mut self, mut span: Span) {
        let start_idx = self
            .spans
            .partition_point(|s| s.to.char_index + 1 < span.from.char_index);
        let end_idx = self
            .spans
            .partition_point(|s| s.from.char_index <= span.to.char_index + 1);

        if start_idx < end_idx {
            span.from = span.from.min(self.spans[start_idx].from);
            span.to = span.to.max(self.spans[end_idx - 1].to);
            self.spans[start_idx] = span;
            if start_idx + 1 < end_idx {
                self.spans.drain((start_idx + 1)..end_idx);
            }
        } else {
            self.spans.insert(start_idx, span);
        }
    }

    pub fn hull(&self) -> Span {
        // Safety: frames are always initialized with at least one span
        #[allow(clippy::unwrap_used)]
        let first = self.spans.first().unwrap();
        if self.spans.len() == 1 {
            return *first;
        }
        #[allow(clippy::unwrap_used)]
        let last = self.spans.last().unwrap();
        Span::new(first.from.min(last.from), first.to.max(last.to))
    }

    pub fn is_hull(&self) -> bool {
        self.spans.len() == 1
    }

    pub fn merge(&mut self, frame: Frame) {
        let new_len = self.spans.len() + frame.spans.len();
        let mut left_iter = std::mem::replace(
            &mut self.spans,
            Vec::<Span>::with_capacity(new_len),
        )
        .into_iter();
        let mut right_iter = frame.spans.into_iter();

        let mut left = left_iter.next();
        let mut right = right_iter.next();

        loop {
            let next_span = match (left, right) {
                (Some(l), Some(r)) => {
                    if l.from.char_index <= r.from.char_index {
                        left = left_iter.next();
                        l
                    } else {
                        right = right_iter.next();
                        r
                    }
                }
                (Some(l), None) => {
                    left = left_iter.next();
                    l
                }
                (None, Some(r)) => {
                    right = right_iter.next();
                    r
                }
                (None, None) => break,
            };

            match self.spans.last_mut() {
                Some(last)
                    if last.to.char_index + 1 >= next_span.from.char_index =>
                {
                    last.to = last.to.max(next_span.to);
                }
                _ => self.spans.push(next_span),
            }
        }
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
}

impl From<Span> for Frame {
    fn from(value: Span) -> Frame {
        Frame::new(value)
    }
}

#[cfg(test)]
mod tests {
    use lexer::Position;

    use super::*;

    #[test]
    fn push_append() {
        let mut frame =
            Frame::new(Span::new(Position::new(0, 0), Position::new(1, 1)));
        frame.push(Span::new(Position::new(3, 3), Position::new(3, 3)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![
                    Span::new(Position::new(0, 0), Position::new(1, 1)),
                    Span::new(Position::new(3, 3), Position::new(3, 3),)
                ]
            }
        )
    }

    #[test]
    fn push_prepend() {
        let mut frame =
            Frame::new(Span::new(Position::new(3, 3), Position::new(4, 4)));
        frame.push(Span::new(Position::new(0, 0), Position::new(1, 1)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![
                    Span::new(Position::new(0, 0), Position::new(1, 1)),
                    Span::new(Position::new(3, 3), Position::new(4, 4)),
                ]
            }
        );
    }

    #[test]
    fn push_insert_between() {
        let mut frame =
            Frame::new(Span::new(Position::new(0, 0), Position::new(1, 1)));
        frame.push(Span::new(Position::new(5, 5), Position::new(6, 6)));
        frame.push(Span::new(Position::new(3, 3), Position::new(3, 3)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![
                    Span::new(Position::new(0, 0), Position::new(1, 1)),
                    Span::new(Position::new(3, 3), Position::new(3, 3)),
                    Span::new(Position::new(5, 5), Position::new(6, 6)),
                ]
            }
        );
    }

    #[test]
    fn push_overlap_left() {
        let mut frame =
            Frame::new(Span::new(Position::new(2, 2), Position::new(4, 4)));
        frame.push(Span::new(Position::new(1, 1), Position::new(3, 3)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(1, 1),
                    Position::new(4, 4)
                ),]
            }
        );
    }

    #[test]
    fn push_overlap_right() {
        let mut frame =
            Frame::new(Span::new(Position::new(1, 1), Position::new(3, 3)));
        frame.push(Span::new(Position::new(2, 2), Position::new(4, 4)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(1, 1),
                    Position::new(4, 4)
                ),]
            }
        );
    }

    #[test]
    fn push_superset() {
        let mut frame =
            Frame::new(Span::new(Position::new(2, 2), Position::new(3, 3)));
        frame.push(Span::new(Position::new(1, 1), Position::new(4, 4)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(1, 1),
                    Position::new(4, 4)
                ),]
            }
        );
    }

    #[test]
    fn push_subset() {
        let mut frame =
            Frame::new(Span::new(Position::new(1, 1), Position::new(4, 4)));
        frame.push(Span::new(Position::new(2, 2), Position::new(3, 3)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(1, 1),
                    Position::new(4, 4)
                ),]
            }
        );
    }

    #[test]
    fn push_merge_multiple() {
        let mut frame =
            Frame::new(Span::new(Position::new(0, 0), Position::new(1, 1)));
        frame.push(Span::new(Position::new(3, 3), Position::new(4, 4)));
        frame.push(Span::new(Position::new(6, 6), Position::new(7, 7)));

        // Push a span that spans across all three
        frame.push(Span::new(Position::new(1, 1), Position::new(6, 6)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(0, 0),
                    Position::new(7, 7)
                ),]
            }
        );
    }

    #[test]
    fn push_exact_match() {
        let mut frame =
            Frame::new(Span::new(Position::new(1, 1), Position::new(2, 2)));
        frame.push(Span::new(Position::new(1, 1), Position::new(2, 2)));
        assert_eq!(
            frame,
            Frame {
                spans: vec![Span::new(
                    Position::new(1, 1),
                    Position::new(2, 2)
                ),]
            }
        );
    }

    #[test]
    fn merge_frames() {
        let mut frame1 = Frame {
            spans: vec![
                Span::new(Position::new(0, 0), Position::new(2, 2)),
                Span::new(Position::new(6, 6), Position::new(8, 8)),
                Span::new(Position::new(14, 14), Position::new(15, 15)),
            ],
        };

        let frame2 = Frame {
            spans: vec![
                Span::new(Position::new(1, 1), Position::new(4, 4)),
                Span::new(Position::new(9, 9), Position::new(10, 10)),
                Span::new(Position::new(12, 12), Position::new(12, 12)),
            ],
        };

        frame1.merge(frame2);

        assert_eq!(
            frame1,
            Frame {
                spans: vec![
                    Span::new(Position::new(0, 0), Position::new(4, 4)),
                    Span::new(Position::new(6, 6), Position::new(10, 10)),
                    Span::new(Position::new(12, 12), Position::new(12, 12)),
                    Span::new(Position::new(14, 14), Position::new(15, 15)),
                ]
            }
        );
    }

    #[test]
    fn hull() {
        let frame = Frame {
            spans: vec![
                Span::new(Position::new(2, 2), Position::new(5, 5)),
                Span::new(Position::new(8, 8), Position::new(10, 10)),
                Span::new(Position::new(15, 15), Position::new(20, 20)),
            ],
        };

        assert_eq!(
            frame.hull(),
            Span::new(Position::new(2, 2), Position::new(20, 20))
        );
    }
}

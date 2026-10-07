use lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    first: Span,
    rest: Vec<Span>,
}

impl Frame {
    pub fn new(span: Span) -> Self {
        Self {
            first: span,
            rest: vec![],
        }
    }

    pub fn push(&mut self, mut span: Span) {
        if self.first.from.char_index > span.to.char_index + 1 {
            // span is left from first with no overlap
            std::mem::swap(&mut self.first, &mut span);
            self.rest.insert(0, span);
        } else if self.first.overlap_or_meets(&span) {
            self.first.to = self.first.to.max(span.to);
            self.first.from = self.first.from.min(span.from);

            while let Some(next) = self.rest.first()
                && next.from.char_index <= self.first.to.char_index + 1
            {
                self.first.to = next.to.max(self.first.to);
                self.rest.remove(0);
            }
        } else {
            let start_idx = self.rest.partition_point(|s| {
                s.to.char_index + 1 < span.from.char_index
            });
            let end_idx = self.rest.partition_point(|s| {
                s.from.char_index <= span.to.char_index + 1
            });

            if start_idx < end_idx {
                span.from = span.from.min(self.rest[start_idx].from);
                span.to = span.to.max(self.rest[end_idx - 1].to);
                self.rest[start_idx] = span;
                if start_idx + 1 < end_idx {
                    self.rest.drain((start_idx + 1)..end_idx);
                }
            } else {
                self.rest.insert(start_idx, span);
            }
        }
    }

    pub fn hull(&self) -> Span {
        let first = self.first;
        if self.rest.is_empty() {
            return first;
        }
        let last = self.rest.last().unwrap_or(&self.first);
        Span::new(first.from.min(last.from), first.to.max(last.to))
    }

    pub fn is_hull(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn merge(&mut self, frame: Frame) {
        self.push(frame.first);
        frame.rest.into_iter().for_each(|span| self.push(span));
    }

    pub fn spans(&self) -> impl Iterator<Item = Span> {
        [self.first].into_iter().chain(self.rest.iter().copied())
    }
}

impl From<Span> for Frame {
    fn from(value: Span) -> Frame {
        Frame::new(value)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
                first: Span::new(Position::new(0, 0), Position::new(1, 1)),
                rest: vec![
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
                first: Span::new(Position::new(0, 0), Position::new(1, 1)),
                rest: vec![Span::new(Position::new(3, 3), Position::new(4, 4)),]
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
                first: Span::new(Position::new(0, 0), Position::new(1, 1)),
                rest: vec![
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
                first: Span::new(Position::new(1, 1), Position::new(4, 4)),
                rest: vec![]
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
                first: Span::new(Position::new(1, 1), Position::new(4, 4)),
                rest: vec![]
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
                first: Span::new(Position::new(1, 1), Position::new(4, 4)),
                rest: vec![]
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
                first: Span::new(Position::new(1, 1), Position::new(4, 4)),
                rest: vec![]
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
                first: Span::new(Position::new(0, 0), Position::new(7, 7)),
                rest: vec![]
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
                first: Span::new(Position::new(1, 1), Position::new(2, 2)),
                rest: vec![]
            }
        );
    }

    #[test]
    fn merge_frames() {
        let mut frame1 = Frame {
            first: Span::new(Position::new(0, 0), Position::new(2, 2)),
            rest: vec![
                Span::new(Position::new(6, 6), Position::new(8, 8)),
                Span::new(Position::new(14, 14), Position::new(15, 15)),
            ],
        };

        let frame2 = Frame {
            first: Span::new(Position::new(1, 1), Position::new(4, 4)),
            rest: vec![
                Span::new(Position::new(9, 9), Position::new(10, 10)),
                Span::new(Position::new(12, 12), Position::new(12, 12)),
            ],
        };

        frame1.merge(frame2);

        assert_eq!(
            frame1,
            Frame {
                first: Span::new(Position::new(0, 0), Position::new(4, 4)),
                rest: vec![
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
            first: Span::new(Position::new(2, 2), Position::new(5, 5)),
            rest: vec![
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

use lexer::Span;

use crate::frame::Frame;

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    frames: Vec<Frame>,
}

impl Trace {
    pub fn new() -> Self {
        Self { frames: vec![] }
    }

    pub fn is_hull(&self) -> bool {
        if let Some(frame) = self.frames.last()
            && frame.is_hull()
            && self.frames.len() == 1
        {
            true
        } else {
            false
        }
    }

    pub fn first(&self) -> Option<&Frame> {
        self.frames.first()
    }

    pub fn first_spans(&self) -> Option<&[Span]> {
        self.first().map(|frame| frame.spans())
    }

    pub fn first_mut(&mut self) -> Option<&mut Frame> {
        self.frames.first_mut()
    }

    pub fn last(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn hull(&self) -> Option<Span> {
        self.frames.last().map(|frame| frame.hull())
    }

    pub fn last_mut(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    /// merge all frames into a single one
    pub fn compact(&mut self) {
        let mut frames =
            std::mem::replace(&mut self.frames, Vec::<Frame>::with_capacity(1));
        let Some(mut frame) = frames.pop() else {
            return;
        };
        frames.into_iter().for_each(|f| {
            frame.merge(f);
        });
        self.frames.push(frame);
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

impl From<Frame> for Trace {
    fn from(value: Frame) -> Self {
        let mut trace = Self::new();
        trace.push(value);
        trace
    }
}

impl From<Span> for Trace {
    fn from(value: Span) -> Trace {
        Trace::from(Frame::from(value))
    }
}

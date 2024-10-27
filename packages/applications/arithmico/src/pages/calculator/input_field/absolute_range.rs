use web_sys::{Node, Range};

use super::{
    node_utils::{absolute_offset, relative_offset},
    static_range::StaticRange,
};

#[derive(Debug, Clone)]
pub struct AbsoluteRange {
    start: usize,
    end: usize,
}

impl AbsoluteRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn into_range(&self, target: &Node) -> Option<Range> {
        let start = relative_offset(&target, self.start);
        let end = relative_offset(&target, self.end);

        match (start, end) {
            (
                Some((start_offset, start_container)),
                Some((end_offset, end_container)),
            ) => {
                let range = Range::new().expect("range");
                range
                    .set_start(&start_container, start_offset as u32)
                    .expect("set start");
                range
                    .set_end(&end_container, end_offset as u32)
                    .expect("set end");
                Some(range)
            }
            _ => None,
        }
    }
}

impl TryFrom<(&Node, &StaticRange)> for AbsoluteRange {
    type Error = &'static str;

    fn try_from(value: (&Node, &StaticRange)) -> Result<Self, Self::Error> {
        let (node, range) = value;
        let start =
            absolute_offset(node, range.start(), &range.start_container());
        let end = absolute_offset(node, range.end(), &range.end_container());

        match (start, end) {
            (Some(start), Some(end)) => Ok(AbsoluteRange::new(start, end)),
            _ => Err("incomplete static range"),
        }
    }
}

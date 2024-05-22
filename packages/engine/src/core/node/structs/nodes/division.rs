use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Division {
    pub dividend: Box<Node>,
    pub divisor: Box<Node>,
}

impl Division {
    pub fn new<L: Into<Node>, R: Into<Node>>(
        dividend: L,
        divisor: R,
    ) -> Division {
        Division {
            dividend: dividend.into().into(),
            divisor: divisor.into().into(),
        }
    }
}

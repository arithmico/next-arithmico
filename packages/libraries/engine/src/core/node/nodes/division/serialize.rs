use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Division;

impl Division {
    fn dividend_requires_parenthesis(&self) -> bool {
        match *self.dividend {
            Node::Negate(_)
            | Node::Sum(_)
            | Node::Function(_)
            | Node::Definition(_)
            | Node::And(_)
            | Node::Or(_) => true,
            _ => false,
        }
    }

    fn divisor_requires_parenthesis(&self) -> bool {
        match *self.dividend {
            Node::Negate(_)
            | Node::Sum(_)
            | Node::Division(_)
            | Node::Function(_)
            | Node::Definition(_)
            | Node::And(_)
            | Node::Or(_) => true,
            _ => false,
        }
    }
}

impl SerializeNode for Division {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        Division::new(
            self.dividend.transform_before_serialization(context),
            self.divisor.transform_before_serialization(context),
        )
        .into()
    }

    fn serialize(&self, context: &Context) -> String {
        let dividend = if self.dividend_requires_parenthesis() {
            self.dividend.serialize_with_parenthesis(context)
        } else {
            self.dividend.serialize(context)
        };
        let divisor = if self.divisor_requires_parenthesis() {
            self.divisor.serialize_with_parenthesis(context)
        } else {
            self.divisor.serialize(context)
        };
        format!("{} / {}", dividend, divisor)
    }
}

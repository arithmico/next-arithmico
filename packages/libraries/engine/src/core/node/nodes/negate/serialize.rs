use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

use super::Negate;

impl Negate {
    fn child_requires_parenthesis(&self) -> bool {
        match *self.value {
            Node::Negate(_)
            | Node::Sum(_)
            | Node::Function(_)
            | Node::Definition(_)
            | Node::And(_)
            | Node::Or(_) => true,
            _ => false,
        }
    }
}

impl SerializeNode for Negate {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        let transformed_value =
            self.value.transform_before_serialization(context);
        Negate::new(transformed_value).into()
    }

    fn serialize(&self, context: &Context) -> String {
        if self.child_requires_parenthesis() {
            format!("-({})", self.value.serialize(context))
        } else {
            format!("-{}", self.value.serialize(context))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::serialization_test;

    #[test]
    fn serialize_negate() {
        serialization_test("-2", "-2");
    }

    #[test]
    fn serialize_nested_negate() {
        serialization_test("-(-2)", "-(-2)");
    }
}

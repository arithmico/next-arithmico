use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, nodes::*, Node},
};

pub fn evaluate_boolean(
    node: &Boolean,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Boolean::new(node.value).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_boolean_true() {
        let context = Context::default();
        assert_eq!(
            Node::from(Boolean::new(true)).evaluate(&context).unwrap(),
            Node::from(Boolean::new(true))
        )
    }

    #[test]
    fn evaluate_boolean_false() {
        let context = Context::default();
        assert_eq!(
            Node::from(Boolean::new(false)).evaluate(&context).unwrap(),
            Node::from(Boolean::new(false))
        )
    }
}

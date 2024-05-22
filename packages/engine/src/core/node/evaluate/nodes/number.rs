use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Node, Number},
};

pub fn evaluate_number(
    node: &Number,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Number::new(node.value).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        let context = Context::default();
        assert_eq!(
            Node::from(Number::new(2.1)).evaluate(&context).unwrap(),
            Node::from(Number::new(2.1))
        )
    }
}

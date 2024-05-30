use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, nodes::*, Node},
};

pub fn evaluate_function(
    node: &Function,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(
        Function::new(node.arguments.clone(), (*node.expression).clone())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_function() {
        let context = Context::default();
        assert_eq!(
            Node::from(Function::new(vec!["x".into()], Symbol::new("x")))
                .evaluate(&context)
                .unwrap(),
            Node::from(Function::new(vec!["x".into()], Symbol::new("x")))
                .into()
        )
    }
}

use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Function, Node},
};

pub fn evaluate_function(
    arguments: &Vec<String>,
    expression: &Node,
    _context: &Context,
) -> Result<Node, NodeEvaluationError> {
    Ok(Function::new(arguments.clone(), expression.clone()).into())
}

#[cfg(test)]
mod tests {
    use crate::core::node::Symbol;

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

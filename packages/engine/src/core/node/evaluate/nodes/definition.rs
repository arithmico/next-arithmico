use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Definition, Node},
};

pub fn evaluate_definition(
    symbol: &String,
    expression: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_expression = expression.evaluate(context)?;
    Ok(Definition::new(symbol.clone(), evaluated_expression).into())
}

#[cfg(test)]
mod tests {
    use crate::core::node::Number;

    use super::*;

    #[test]
    fn evaluate_definition() {
        let context = Context::default();
        assert_eq!(
            Node::from(Definition::new("test", Number::new(42.0)))
                .evaluate(&context)
                .unwrap(),
            Definition::new("test", Number::new(42.0)).into()
        )
    }
}

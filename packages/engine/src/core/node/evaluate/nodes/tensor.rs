use crate::core::{
    context::Context,
    node::{evaluate::NodeEvaluationError, Node, Tensor},
};

pub fn evaluate_tensor(
    node: &Tensor,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let mut evaluated_values = Vec::<Node>::new();
    for value in &node.elements {
        let evaluated_value = value.evaluate(context)?;
        evaluated_values.push(evaluated_value);
    }
    Ok(Tensor::new(evaluated_values).into())
}

#[cfg(test)]
mod tests {
    use crate::core::node::{Number, Sum};

    use super::*;

    #[test]
    fn evaluate_vector() {
        let context = Context::default();
        assert_eq!(
            Node::from(Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Sum::new(vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                ])
                .into()
            ]))
            .evaluate(&context)
            .unwrap(),
            Tensor::new(vec![
                Number::new(1.0).into(),
                Number::new(2.0).into(),
                Number::new(3.0).into(),
            ])
            .into()
        )
    }
}

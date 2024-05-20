use crate::{context::Context, evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_vector(
    values: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let mut evaluated_values = Vec::<Node>::new();
    for value in values {
        let evaluated_value = value.evaluate(context)?;
        evaluated_values.push(evaluated_value);
    }
    Ok(Node::Vector {
        values: evaluated_values,
    })
}

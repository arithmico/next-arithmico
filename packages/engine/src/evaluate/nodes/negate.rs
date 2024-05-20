use crate::context::Context;
use crate::{evaluate::NodeEvaluationError, node::Node};

pub fn evaluate_negate(
    value: &Node,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_value = value.evaluate(context)?;
    match evaluated_value {
        Node::Number { value } if cfg!(feature = "operator_negate_number") => {
            Ok(Node::Number { value: -value })
        }
        Node::Vector { values } if cfg!(feature = "operator_negate_vector") => {
            let mut negated_values = Vec::<Node>::new();
            for value in values {
                negated_values.push(evaluate_negate(&value, context)?);
            }
            Ok(Node::Vector {
                values: negated_values,
            })
        }
        _ => Err(NodeEvaluationError::UnsupportedOperation),
    }
}

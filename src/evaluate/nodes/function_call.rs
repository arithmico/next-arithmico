use crate::{context::Context, evaluate::EvaluationError, node::Node};

pub fn evaluate_function_call(
    target: &Node,
    call_arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, EvaluationError> {
    let evaluated_target = target.evaluate(context)?;
    match evaluated_target {
        Node::Function {
            arguments,
            expression,
        } => {
            if call_arguments.len() != arguments.len() {
                return Err(EvaluationError::InvalidNumberOfArguments);
            }
            let mut call_context = context.clone();
            call_context.push_frame();
            for (index, call_argument) in call_arguments.iter().enumerate() {
                let evaluated_call_argument =
                    call_argument.evaluate(context)?;
                call_context.insert(
                    arguments.get(index).unwrap(),
                    evaluated_call_argument,
                );
            }
            expression.evaluate(&call_context)
        }
        _ => Err(EvaluationError::UnsupportedOperation),
    }
}

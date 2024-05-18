use crate::{
    context::{Context, HostEndpoint},
    evaluate::NodeEvaluationError,
    node::Node,
};

pub fn evaluate_function_call(
    target: &Node,
    call_arguments: &Vec<Node>,
    context: &Context,
) -> Result<Node, NodeEvaluationError> {
    let evaluated_target = target.evaluate(context)?;
    match evaluated_target {
        Node::Function {
            arguments,
            expression,
        } => {
            if call_arguments.len() != arguments.len() {
                return Err(NodeEvaluationError::InvalidNumberOfArguments);
            }
            let mut call_stack = context.stack.clone();
            call_stack.add_frame();
            for (index, call_argument) in call_arguments.iter().enumerate() {
                let evaluated_call_argument =
                    call_argument.evaluate(context)?;
                call_stack.insert(
                    arguments.get(index).unwrap(),
                    evaluated_call_argument,
                );
            }
            let call_context = Context::new(
                call_stack,
                context.settings.clone(),
                context.host_api.clone(),
            );
            expression.evaluate(&call_context)
        }
        Node::HostApiFunctionEndpoint { name } => {
            let endpoint = context.endpoint(&name).unwrap();
            match endpoint {
                HostEndpoint::Function { executor, .. } => {
                    executor(&call_arguments, context)
                }
                HostEndpoint::Constant { .. } => {
                    Err(NodeEvaluationError::RuntimeError(
                        "Can not call constant".into(),
                    ))
                }
            }
        }
        _ => Err(NodeEvaluationError::UnsupportedOperation),
    }
}

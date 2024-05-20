use crate::core::{
    context::Context, evaluate::NodeEvaluationError, host_api::HostEndpoint,
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

#[cfg(test)]
mod tests {
    use crate::{
        core::{context::Stack, host_api::HostApiModule},
        language::Language,
        HostApi, Settings,
    };

    use super::*;

    #[test]
    fn evaluate_function_call() {
        let context = Context::default();
        assert_eq!(
            Node::FunctionCall {
                target: Box::new(Node::Function {
                    arguments: vec![String::from("x")],
                    expression: Box::new(Node::Symbol {
                        name: String::from("x")
                    })
                }),
                arguments: vec![Node::Number { value: 42.0 }]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Number { value: 42.0 }
        )
    }

    #[test]
    fn evaluate_host_function_endpoint() {
        let host_api = HostApi::builder()
            .module(true, || {
                HostApiModule::builder()
                    .name("test")
                    .endpoint(true, "test", |builder| {
                        builder
                            .description(Language::English, "test")
                            .function(vec!["x"])
                            .executor(|arguments, context| {
                                let mut result: Vec<Node> = Vec::new();
                                for argument in arguments.iter() {
                                    let evaluated_argument =
                                        argument.evaluate(context)?;
                                    result.push(evaluated_argument);
                                }
                                Ok(Node::Vector { values: result })
                            })
                    })
                    .build()
            })
            .build();

        let context =
            Context::new(Stack::new(), Settings::default(), host_api.into());

        assert_eq!(
            Node::FunctionCall {
                target: Node::Symbol {
                    name: "test".into()
                }
                .into(),
                arguments: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            }
            .evaluate(&context)
            .unwrap(),
            Node::Vector {
                values: vec![
                    Node::Number { value: 1.0 },
                    Node::Number { value: 2.0 },
                    Node::Number { value: 3.0 },
                ]
            }
        )
    }
}

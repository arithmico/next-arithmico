use crate::core::{context::Context, host_api::HostEndpoint, node::*};

impl EvaluateNode for FunctionCall {
    fn evaluate(&self, context: &Context) -> Result<Node, NodeError> {
        let evaluated_target = self.target.evaluate(context)?;
        match evaluated_target {
            Node::Function(Function {
                arguments,
                expression,
            }) => {
                if self.arguments.len() != arguments.len() {
                    return Err(NodeError::RuntimeError(String::from(
                        "invalid number of arguments",
                    )));
                }
                let mut call_stack = context.stack.clone();
                call_stack.add_frame();
                for (index, call_argument) in self.arguments.iter().enumerate()
                {
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
            Node::HostApiFunctionEndpoint(HostFunction { name }) => {
                let endpoint = context.endpoint(&name).unwrap();
                match endpoint {
                    HostEndpoint::Function { executor, .. } => {
                        executor(&self.arguments, context)
                    }
                    HostEndpoint::Constant { .. } => Err(
                        NodeError::RuntimeError("Can not call constant".into()),
                    ),
                }
            }
            _ => Err(NodeError::UnsupportedOperation),
        }
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
            Node::from(FunctionCall::new(
                Function::new(vec!["x".into()], Symbol::new("x")),
                vec![Number::new(42.0).into()]
            ))
            .evaluate(&context)
            .unwrap(),
            Number::new(42.0).into()
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
                                Ok(Tensor::new(result).into())
                            })
                    })
                    .build()
            })
            .build();

        let context =
            Context::new(Stack::new(), Settings::default(), host_api.into());

        assert_eq!(
            Node::from(FunctionCall::new(
                Symbol::new("test"),
                vec![
                    Number::new(1.0).into(),
                    Number::new(2.0).into(),
                    Number::new(3.0).into(),
                ]
            ))
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

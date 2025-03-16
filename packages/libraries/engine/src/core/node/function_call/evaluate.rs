use crate::core::{
    map_function_parameters, EvaluateNode, EvaluateNodeContext,
    EvaluateNodeError, FunctionCall, HostEndpoint, Node,
};

impl EvaluateNode for FunctionCall {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        let target = self.target.evaluate(context)?;

        match target {
            Node::Function(function) => {
                let mapping = map_function_parameters(
                    &function.signature,
                    &self.arguments,
                    context,
                )?;

                let mut stack = context.stack.clone();
                stack.add_frame();

                for name in mapping.parameter_names() {
                    let value = mapping.get_parameter_value(&name)?;
                    stack.insert(&name, value);
                }

                let local_context = EvaluateNodeContext::new(
                    stack,
                    context.decimal_places.clone(),
                    context.decimal_format.clone(),
                    context.host_api.clone(),
                );

                function.expression.evaluate(&local_context)
            }
            Node::HostFunction(host_function) => {
                let Some(endpoint) =
                    context.host_api.endpoint(&host_function.name)
                else {
                    return Err(EvaluateNodeError::unknown_symbol(
                        host_function.name,
                    ));
                };

                let HostEndpoint::Function {
                    executor,
                    signature,
                    ..
                } = endpoint
                else {
                    return Err(EvaluateNodeError::unsupported_operation());
                };

                let mapping = map_function_parameters(
                    &signature,
                    &self.arguments,
                    context,
                )?;

                executor(&mapping, context)
            }
            node => {
                Err(EvaluateNodeError::unsupported_operation()
                    .with_tracable(node))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        core::{
            Function, FunctionSignature, HostApi, HostApiModule, Language,
            NodeType, Number, Power, Stack, Symbol,
        },
        DecimalFormat, DecimalPlaces,
    };

    use super::*;

    #[test]
    fn evaluate_function_call_with_invalid_number_of_arguments() {
        let context = EvaluateNodeContext::default();
        let result = FunctionCall::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            vec![],
        )
        .evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::missing_parameter("x")));
    }

    #[test]
    fn evaluate_function_call() {
        let context = EvaluateNodeContext::default();
        let result = FunctionCall::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            vec![Number::new(2.)],
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(2.));
    }

    #[test]
    fn evaluate_host_function_call() {
        let context = EvaluateNodeContext::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .id("test")
                            .name(Language::English, "test")
                            .endpoint(true, "f", |builder| {
                                builder
                                    .description(Language::English, "test")
                                    .function(
                                        FunctionSignature::new()
                                            .argument("x", |argument| {
                                                argument
                                                    .node_type(NodeType::Any)
                                            })
                                            .add_return_type(NodeType::Any),
                                    )
                                    .executor(|arguments, context| {
                                        Power::new(
                                            arguments
                                                .get_parameter_value("x")?,
                                            Number::new(2.),
                                        )
                                        .evaluate(context)
                                    })
                            })
                            .build()
                    })
                    .build(),
            ),
        );
        let result = FunctionCall::new(Symbol::new("f"), vec![Number::new(2.)])
            .evaluate(&context)
            .unwrap();
        assert_eq!(result, Number::new(4.));
    }

    #[test]
    fn evaluate_host_function_call_invalid_number_of_arguments() {
        let context = EvaluateNodeContext::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .id("test")
                            .name(Language::English, "test")
                            .endpoint(true, "f", |builder| {
                                builder
                                    .description(Language::English, "test")
                                    .function(
                                        FunctionSignature::new()
                                            .argument("x", |argument| {
                                                argument
                                                    .node_type(NodeType::Any)
                                            })
                                            .add_return_type(NodeType::Any),
                                    )
                                    .executor(|arguments, context| {
                                        Power::new(
                                            arguments
                                                .get_parameter_value("x")?,
                                            Number::new(2.),
                                        )
                                        .evaluate(context)
                                    })
                            })
                            .build()
                    })
                    .build(),
            ),
        );
        let result =
            FunctionCall::new(Symbol::new("f"), vec![]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::missing_parameter("x")));
    }
}

use node::{FunctionCall, Node};

use crate::core::{
    map_function_parameters, Context, EvaluateNode, EvaluateNodeError,
    HostEndpoint,
};

impl EvaluateNode for FunctionCall {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
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
                    let value = mapping.get_node(&name)?;
                    stack.insert(&name, value);
                }

                let local_context = Context::new(
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

    use engine_derive::FromArgumentMapping;
    use node::{
        Boolean, Function, FunctionSignature, NodeType, Number, Power, Symbol,
    };

    use crate::{
        core::{HostApi, HostApiModule, Language, Stack},
        function_executor_wrapper, DecimalFormat, DecimalPlaces,
    };

    use super::*;

    #[test]
    fn evaluate_function_call_with_invalid_number_of_arguments() {
        let context = Context::default();
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
        let context = Context::default();
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
        #[derive(FromArgumentMapping)]
        struct TestArgs<'a> {
            x: &'a Number,
        }

        fn test_executor(
            TestArgs { x }: TestArgs,
            context: &Context,
        ) -> Result<Node, EvaluateNodeError> {
            Power::new(Number::new(x.value), Number::new(2.)).evaluate(context)
        }

        let context = Context::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .id("test")
                            .name(Language::English, "test")
                            .endpoints(&[|builder| {
                                builder
                                    .name("f")
                                    .description(Language::English, "test")
                                    .function(
                                        FunctionSignature::new()
                                            .argument("x", |argument| {
                                                argument
                                                    .node_type(NodeType::Any)
                                            })
                                            .add_return_type(NodeType::Any),
                                    )
                                    .executor(function_executor_wrapper!(
                                        TestArgs,
                                        test_executor
                                    ))
                            }])
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
        #[derive(FromArgumentMapping)]
        struct TestArgs<'a> {
            x: &'a Number,
        }

        fn test_executor(
            TestArgs { x }: TestArgs,
            context: &Context,
        ) -> Result<Node, EvaluateNodeError> {
            Power::new(Number::new(x.value), Number::new(2.)).evaluate(context)
        }

        let context = Context::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .id("test")
                            .name(Language::English, "test")
                            .endpoints(&[|builder| {
                                builder
                                    .name("f")
                                    .description(Language::English, "test")
                                    .function(
                                        FunctionSignature::new()
                                            .argument("x", |argument| {
                                                argument
                                                    .node_type(NodeType::Any)
                                            })
                                            .add_return_type(NodeType::Any),
                                    )
                                    .executor(function_executor_wrapper!(
                                        TestArgs,
                                        test_executor
                                    ))
                            }])
                            .build()
                    })
                    .build(),
            ),
        );
        let result =
            FunctionCall::new(Symbol::new("f"), vec![]).evaluate(&context);
        assert_eq!(result, Err(EvaluateNodeError::missing_parameter("x")));
    }

    #[test]
    fn evaluate_host_function_call_with_advanced_arguments() {
        #[derive(FromArgumentMapping)]
        struct TestArgs<'a> {
            a: &'a Number,
            b: Option<&'a Boolean>,
            c: Vec<&'a Number>,
        }

        fn test_executor(
            TestArgs { a, b, c }: TestArgs,
            context: &Context,
        ) -> Result<Node, EvaluateNodeError> {
            let result = if let Some(_) = b {
                Number::new(a.value)
            } else {
                Number::new(c.iter().map(|n| n.value).sum())
            };

            result.evaluate(context)
        }

        let context = Context::new(
            Stack::new(),
            DecimalPlaces::default(),
            DecimalFormat::default(),
            Arc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .id("test")
                            .name(Language::English, "test")
                            .endpoints(&[|builder| {
                                builder
                                    .name("f")
                                    .description(Language::English, "test")
                                    .function(
                                        FunctionSignature::new()
                                            .argument("a", |argument| {
                                                argument
                                                    .node_type(NodeType::Any)
                                            })
                                            .argument("b", |argument| {
                                                argument
                                                    .optional()
                                                    .node_type(NodeType::Any)
                                            })
                                            .argument("c", |argument| {
                                                argument
                                                    .repeatable()
                                                    .node_type(NodeType::Any)
                                            })
                                            .add_return_type(NodeType::Any),
                                    )
                                    .executor(function_executor_wrapper!(
                                        TestArgs,
                                        test_executor
                                    ))
                            }])
                            .build()
                    })
                    .build(),
            ),
        );
        let result = FunctionCall::new(
            Symbol::new("f"),
            vec![
                Number::new(5.),
                Boolean::new(false),
                Number::new(1.),
                Number::new(2.),
                Number::new(3.),
            ],
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(5.));
    }
}

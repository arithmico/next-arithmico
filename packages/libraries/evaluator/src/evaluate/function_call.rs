use node::{FunctionCall, Node};
use trace::{Tracable, TracableMut};

use crate::{ArgumentMapping, Endpoint, Error, EvaluateNode, Options};

impl EvaluateNode for FunctionCall {
    fn evaluate(&self, options: Options) -> Result<Node, Error> {
        let target = self.target.evaluate(options)?;

        match target {
            Node::Function(function) => {
                let mapping = ArgumentMapping::from_arguments(
                    &function.signature,
                    &self.arguments,
                    options,
                )?;

                let mut stack = options.stack.clone();
                stack.add_frame();

                for name in mapping.parameter_names() {
                    let value = mapping.get_node(&name)?;
                    stack.insert(&name, value);
                }

                let local_options =
                    Options::new(&stack, &options.api, options.angle_unit);

                function.expression.evaluate(local_options)
            }
            Node::HostFunction(host_function) => {
                let Some(endpoint) = options.api.endpoint(&host_function.name)
                else {
                    return Err(Error::unknown_symbol(host_function.name));
                };

                let Endpoint::Function {
                    executor,
                    signature,
                    ..
                } = endpoint
                else {
                    return Err(Error::unsupported_operation());
                };

                let mapping = ArgumentMapping::from_arguments(
                    &signature,
                    &self.arguments,
                    options,
                )?;

                executor(&mapping, options)
            }
            node => {
                Err(Error::unsupported_operation()
                    .with_optional_span(node.hull()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use engine_derive::FunctionArguments;
    use node::{
        Boolean, Function, FunctionSignature, NodeType, Number, Power, Symbol,
    };
    use translate_core::Language;

    use crate::{Api, ApiModule, FunctionArguments, Stack};

    use super::*;

    #[test]
    fn evaluate_function_call_with_invalid_number_of_arguments() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = FunctionCall::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            vec![],
        )
        .evaluate(options);
        assert_eq!(result, Err(Error::missing_parameter("x")));
    }

    #[test]
    fn evaluate_function_call() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = FunctionCall::new(
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x"),
            ),
            vec![Number::new_node(2.)],
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Number::new_node(2.));
    }

    #[test]
    fn evaluate_host_function_call() {
        #[derive(FunctionArguments)]
        #[name("test")]
        struct TestArgs<'a> {
            x: &'a Number,
        }

        fn test_executor(
            TestArgs { x }: TestArgs,
            options: Options,
        ) -> Result<Node, Error> {
            Power::new(Number::new_node(x.value), Number::new_node(2.))
                .evaluate(options)
        }

        let stack = Stack::new();
        let api = Api::builder()
            .module(true, || {
                ApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoints(&[|builder| {
                        builder
                            .name("f")
                            .description(Language::English, "test")
                            .function(
                                FunctionSignature::new()
                                    .argument("x", |argument| {
                                        argument.node_type(NodeType::Any)
                                    })
                                    .add_return_type(NodeType::Any),
                            )
                            .executor(|argument, context| {
                                let args = TestArgs::from_mapping(&argument)?;
                                test_executor(args, context)
                            })
                    }])
                    .build()
            })
            .build();

        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result =
            FunctionCall::new(Symbol::new("f"), vec![Number::new_node(2.)])
                .evaluate(options)
                .unwrap();
        assert_eq!(result, Number::new_node(4.));
    }

    #[test]
    fn evaluate_host_function_call_invalid_number_of_arguments() {
        #[derive(FunctionArguments)]
        #[name("test")]
        struct TestArgs<'a> {
            x: &'a Number,
        }

        fn test_executor(
            TestArgs { x }: TestArgs,
            options: Options,
        ) -> Result<Node, Error> {
            Power::new(Number::new_node(x.value), Number::new_node(2.))
                .evaluate(options)
        }

        let stack = Stack::new();
        let api = Api::builder()
            .module(true, || {
                ApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoints(&[|builder| {
                        builder
                            .name("f")
                            .description(Language::English, "test")
                            .function(
                                FunctionSignature::new()
                                    .argument("x", |argument| {
                                        argument.node_type(NodeType::Any)
                                    })
                                    .add_return_type(NodeType::Any),
                            )
                            .executor(|argument, context| {
                                let args = TestArgs::from_mapping(&argument)?;
                                test_executor(args, context)
                            })
                    }])
                    .build()
            })
            .build();

        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result =
            FunctionCall::new(Symbol::new("f"), vec![]).evaluate(options);
        assert_eq!(result, Err(Error::missing_parameter("x")));
    }

    #[test]
    fn evaluate_host_function_call_with_advanced_arguments() {
        #[derive(FunctionArguments)]
        #[name("test")]
        struct TestArgs<'a> {
            a: &'a Number,
            b: Option<&'a Boolean>,
            c: Vec<&'a Number>,
        }

        fn test_executor(
            TestArgs { a, b, c }: TestArgs,
            context: Options,
        ) -> Result<Node, Error> {
            let result = if let Some(_) = b {
                Number::new_node(a.value)
            } else {
                Number::new_node(c.iter().map(|n| n.value).sum())
            };

            result.evaluate(context)
        }

        let stack = Stack::new();
        let api = Api::builder()
            .module(true, || {
                ApiModule::builder()
                    .id("test")
                    .name(Language::English, "test")
                    .endpoints(&[|builder| {
                        builder
                            .name("f")
                            .description(Language::English, "test")
                            .function(
                                FunctionSignature::new()
                                    .argument("a", |argument| {
                                        argument.node_type(NodeType::Any)
                                    })
                                    .argument("b", |argument| {
                                        argument
                                            .optional()
                                            .node_type(NodeType::Any)
                                    })
                                    .argument("c", |argument| {
                                        argument
                                            .repeatable(1_usize, None)
                                            .node_type(NodeType::Any)
                                    })
                                    .add_return_type(NodeType::Any),
                            )
                            .executor(|argument, context| {
                                let args = TestArgs::from_mapping(&argument)?;
                                test_executor(args, context)
                            })
                    }])
                    .build()
            })
            .build();

        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = FunctionCall::new(
            Symbol::new("f"),
            vec![
                Number::new_node(5.),
                Boolean::new(false),
                Number::new_node(1.),
                Number::new_node(2.),
                Number::new_node(3.),
            ],
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(result, Number::new_node(5.));
    }
}

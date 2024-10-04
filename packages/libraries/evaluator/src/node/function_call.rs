use std::iter::zip;

use ast::{FunctionCall, Node};

use crate::{
    context::HostEndpoint, evaluate::EvaluateNode, EvaluateNodeContext,
    EvaluateNodeError,
};

impl EvaluateNode for FunctionCall {
    fn evaluate(
        &self,
        context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        let target = self.target.evaluate(context)?;

        match target {
            Node::Function(function) => {
                if self.arguments.len() != function.arguments.len() {
                    return Err(EvaluateNodeError::InvalidNumberOfArguments(
                        function.arguments.len(),
                        self.arguments.len(),
                    ));
                }

                let mut stack = context.stack.clone();
                stack.add_frame();

                for (name, value) in
                    zip(function.arguments.iter(), self.arguments.iter())
                {
                    stack.insert(name, value.clone());
                }

                let local_context = EvaluateNodeContext::new(
                    stack,
                    context.options.clone(),
                    context.host_api.clone(),
                );

                function.expression.evaluate(&local_context)
            }
            Node::HostFunction(host_function) => {
                let Some(endpoint) =
                    context.host_api.endpoint(&host_function.name)
                else {
                    return Err(EvaluateNodeError::UnknownSymbol(
                        host_function.name,
                    ));
                };

                let HostEndpoint::Function {
                    executor,
                    arguments,
                    ..
                } = endpoint
                else {
                    return Err(EvaluateNodeError::UnsupportedOperation);
                };

                if self.arguments.len() != arguments.len() {
                    return Err(EvaluateNodeError::InvalidNumberOfArguments(
                        arguments.len(),
                        self.arguments.len(),
                    ));
                }

                executor(&self.arguments, context)
            }
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ast::{Function, Number, Power, Symbol};

    use crate::{EvaluateNodeOptions, HostApi, HostApiModule, Stack};

    use super::*;

    #[test]
    fn evaluate_function_call_with_invalid_number_of_arguments() {
        let context = EvaluateNodeContext::default();
        let result = FunctionCall::new(
            Function::new(vec![String::from("x")], Symbol::new("x")),
            vec![],
        )
        .evaluate(&context);
        assert_eq!(
            result,
            Err(EvaluateNodeError::InvalidNumberOfArguments(1, 0))
        );
    }

    #[test]
    fn evaluate_function_call() {
        let context = EvaluateNodeContext::default();
        let result = FunctionCall::new(
            Function::new(vec![String::from("x")], Symbol::new("x")),
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
            EvaluateNodeOptions::default(),
            Rc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .name("test")
                            .endpoint(true, "f", |builder| {
                                builder
                                    .description(
                                        common::Language::English,
                                        "test",
                                    )
                                    .function(vec!["x"])
                                    .executor(|arguments, context| {
                                        Power::new(
                                            arguments.get(0).ok_or(
                                                EvaluateNodeError::RuntimeError(
                                                    String::from("test"),
                                                ),
                                            )?.clone(),
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
            EvaluateNodeOptions::default(),
            Rc::new(
                HostApi::builder()
                    .module(true, || {
                        HostApiModule::builder()
                            .name("test")
                            .endpoint(true, "f", |builder| {
                                builder
                                    .description(
                                        common::Language::English,
                                        "test",
                                    )
                                    .function(vec!["x"])
                                    .executor(|arguments, context| {
                                        Power::new(
                                            arguments.get(0).ok_or(
                                                EvaluateNodeError::RuntimeError(
                                                    String::from("test"),
                                                ),
                                            )?.clone(),
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
        assert_eq!(
            result,
            Err(EvaluateNodeError::InvalidNumberOfArguments(1, 0))
        );
    }
}

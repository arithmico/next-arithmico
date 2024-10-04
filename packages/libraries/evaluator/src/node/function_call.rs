use std::iter::zip;

use ast::{FunctionCall, Node};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for FunctionCall {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
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

                let local_context = Context::new(
                    stack,
                    context.settings.clone(),
                    context.host_api.clone(),
                );

                function.expression.evaluate(&local_context)
            }
            _ => Err(EvaluateNodeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::{Function, Number, Symbol};

    use super::*;

    #[test]
    fn evaluate_function_call_with_invalid_number_of_arguments() {
        let context = Context::default();
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
        let context = Context::default();
        let result = FunctionCall::new(
            Function::new(vec![String::from("x")], Symbol::new("x")),
            vec![Number::new(2.)],
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(result, Number::new(2.));
    }
}

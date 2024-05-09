use pest::error::Error;
use thiserror::Error;

use crate::{
    context::Context,
    evaluate::NodeEvaluationError,
    node::Node,
    parser::{parse::parse_statement, parser::Rule},
};

#[derive(Error, Debug, PartialEq)]
pub enum EvaluationError {
    #[error("SyntaxError: {0}")]
    SyntaxError(Error<Rule>),

    #[error("RuntimeError: {0}")]
    RuntimeError(NodeEvaluationError),
}

impl From<Error<Rule>> for EvaluationError {
    fn from(value: Error<Rule>) -> Self {
        EvaluationError::SyntaxError(value)
    }
}

impl From<NodeEvaluationError> for EvaluationError {
    fn from(value: NodeEvaluationError) -> Self {
        EvaluationError::RuntimeError(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    input: String,
    output: Result<String, EvaluationError>,
    context: Context,
}

#[derive(Debug, PartialEq)]
pub struct Session {
    current_context: Context,
    statements: Vec<Statement>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            current_context: Context::new(),
            statements: Vec::new(),
        }
    }

    fn evaluate_input(
        &self,
        input: &str,
    ) -> Result<(String, Context), EvaluationError> {
        let node = parse_statement(input)?;
        let result = node.evaluate(&self.current_context)?;
        let serialized_result = result.serialize();
        let mut next_context = self.current_context.clone();
        if let Node::Definition { symbol, expression } = result {
            next_context.insert(&symbol, (*expression).clone());
        }
        Ok((serialized_result, next_context))
    }

    pub fn push(&mut self, input: &str) {
        let output_result = self.evaluate_input(input);
        let statement = match output_result {
            Ok((output, context)) => {
                let statement = Statement {
                    input: String::from(input),
                    output: Ok(output),
                    context: self.current_context.clone(),
                };
                self.current_context = context;
                statement
            }
            Err(error) => Statement {
                input: String::from(input),
                output: Err(error),
                context: self.current_context.clone(),
            },
        };
        self.statements.push(statement);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_statement() {
        let mut session = Session::new();
        session.push("1+2");
        assert_eq!(
            session,
            Session {
                current_context: Context::new(),
                statements: vec![Statement {
                    input: "1+2".into(),
                    output: Ok("3".into()),
                    context: Context::new(),
                }]
            }
        )
    }

    #[test]
    fn define_symbol_and_evaluate_later() {
        let mut session = Session::new();
        session.push("a := 2");
        session.push("a");
        let mut context_after_definition = Context::new();
        context_after_definition.insert("a", Node::Number { value: 2.0 });
        assert_eq!(
            session,
            Session {
                current_context: context_after_definition.clone(),
                statements: vec![
                    Statement {
                        input: "a := 2".into(),
                        output: Ok("a := 2".into()),
                        context: Context::new(),
                    },
                    Statement {
                        input: "a".into(),
                        output: Ok("2".into()),
                        context: context_after_definition,
                    }
                ]
            }
        )
    }
}

use pest::error::Error;
use thiserror::Error;

use crate::{
    context::Context,
    evaluate::NodeEvaluationError,
    node::Node,
    parser::{parse::parse_statement, parser::Rule},
};

#[derive(Error, Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub input: String,
    pub output: Result<String, EvaluationError>,
    context: Context,
}

#[derive(Debug, PartialEq, Clone)]
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
        let serialized_result = result.serialize(&self.current_context);
        let mut next_context = self.current_context.clone();
        if let Node::Definition { symbol, expression } = result {
            next_context.insert(&symbol, (*expression).clone());
        }
        Ok((serialized_result, next_context))
    }

    pub fn push(&self, input: &str) -> Self {
        let output_result = self.evaluate_input(input);
        let (context, statement) = match output_result {
            Ok((output, context)) => {
                let statement = Statement {
                    input: String::from(input),
                    output: Ok(output),
                    context: self.current_context.clone(),
                };
                (context, statement)
            }
            Err(error) => (
                self.current_context.clone(),
                Statement {
                    input: String::from(input),
                    output: Err(error),
                    context: self.current_context.clone(),
                },
            ),
        };
        let mut next_statements = self.statements.clone();
        next_statements.push(statement);
        Session {
            current_context: context,
            statements: next_statements,
        }
    }

    pub fn statement(&self, index: usize) -> Option<&Statement> {
        self.statements.get(index)
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn last_statement(&self) -> Option<&Statement> {
        self.statements.last()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_statement() {
        let session = Session::new();
        assert_eq!(
            session.push("1+2"),
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
        let session = Session::new();
        let mut context_after_definition = Context::new();
        context_after_definition.insert("a", Node::Number { value: 2.0 });
        assert_eq!(
            session.push("a := 2").push("a"),
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

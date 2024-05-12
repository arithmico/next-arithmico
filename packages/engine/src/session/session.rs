use std::rc::Rc;

use crate::{
    context::{Context, HostApi},
    node::Node,
    parse::parse::parse_statement,
};

use super::EvaluationError;

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub input: String,
    pub output: Result<String, EvaluationError>,
    context: Rc<Context>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Session {
    current_context: Rc<Context>,
    statements: Vec<Statement>,
}

impl Session {
    pub fn new(host_api: Rc<HostApi>) -> Session {
        Session {
            current_context: Context::new(host_api).into(),
            statements: Vec::new(),
        }
    }

    fn evaluate_input(
        &self,
        input: &str,
    ) -> Result<(String, Rc<Context>), EvaluationError> {
        let node = parse_statement(input)?;
        let result = node.evaluate(&self.current_context)?;
        let serialized_result = result.serialize(&self.current_context);
        if let Node::Definition { symbol, expression } = result {
            let mut next_context = (*self.current_context).clone();
            next_context.insert(&symbol, (*expression).clone());
            return Ok((serialized_result, next_context.into()));
        }
        Ok((serialized_result, self.current_context.clone()))
    }

    pub fn push(&self, input: &str) -> Session {
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
        let host_api = Rc::new(HostApi::builder().build());
        let session = Session::new(host_api.clone());
        assert_eq!(
            session.push("1+2"),
            Session {
                current_context: Context::new(host_api.clone()).into(),
                statements: vec![Statement {
                    input: "1+2".into(),
                    output: Ok("3".into()),
                    context: Context::new(host_api.clone()).into(),
                }]
            }
        )
    }

    #[test]
    fn define_symbol_and_evaluate_later() {
        let host_api = Rc::new(HostApi::builder().build());
        let session = Session::new(host_api.clone());
        let mut context_after_definition = Context::new(host_api.clone());
        context_after_definition.insert("a", Node::Number { value: 2.0 });
        assert_eq!(
            session.push("a := 2").push("a"),
            Session {
                current_context: context_after_definition.clone().into(),
                statements: vec![
                    Statement {
                        input: "a := 2".into(),
                        output: Ok("a := 2".into()),
                        context: Context::new(host_api.into()).into(),
                    },
                    Statement {
                        input: "a".into(),
                        output: Ok("2".into()),
                        context: context_after_definition.into(),
                    }
                ]
            }
        )
    }
}

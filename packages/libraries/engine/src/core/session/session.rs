use std::{collections::HashMap, rc::Rc};

use crate::core::{
    context::{Context, Settings, Stack},
    host_api::HostApi,
    node::*,
};

use super::EvaluationError;

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub input: String,
    pub output: Result<String, EvaluationError>,
    pub stack: Stack,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Session {
    stack: Stack,
    host_api: Rc<HostApi>,
    statements: Vec<Statement>,
}

impl Session {
    pub fn new(host_api: Rc<HostApi>) -> Session {
        let stack: Stack = Stack::new();
        Session {
            host_api,
            stack,
            statements: Vec::new(),
        }
    }

    fn get_context(&self, settings: &Settings) -> Context {
        Context::new(
            self.stack.clone(),
            settings.clone(),
            self.host_api.clone(),
        )
    }

    fn evaluate_input(
        &self,
        input: &str,
        settings: &Settings,
    ) -> Result<(String, Stack), EvaluationError> {
        let node = Node::parse(input)?;
        let context = self.get_context(settings);
        let result = node.evaluate(&context)?;
        let serialized_result = result.serialize(&context);
        if let Node::Definition(Definition { symbol, expression }) = result {
            let mut next_stack = self.stack.clone();
            next_stack.insert(&symbol, (*expression).clone());
            return Ok((serialized_result, next_stack));
        }
        Ok((serialized_result, self.stack.clone()))
    }

    pub fn push(&self, input: &str, settings: &Settings) -> Session {
        let output_result = self.evaluate_input(input, settings);
        let (stack, statement) = match output_result {
            Ok((output, stack)) => {
                let statement = Statement {
                    input: String::from(input),
                    output: Ok(output),
                    stack: self.stack.clone(),
                };
                (stack, statement)
            }
            Err(error) => (
                self.stack.clone(),
                Statement {
                    input: String::from(input),
                    output: Err(error),
                    stack: self.stack.clone(),
                },
            ),
        };
        let mut next_statements = self.statements.clone();
        next_statements.push(statement);
        Session {
            stack,
            statements: next_statements,
            host_api: self.host_api.clone(),
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

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn get_definitions(
        &self,
        settings: &Settings,
    ) -> HashMap<String, String> {
        let context = self.get_context(settings);
        let mut result = HashMap::new();
        for (key, value) in self.stack.entries() {
            match value {
                Node::Function(function) => {
                    let mut key = key.clone();
                    key.push_str("(");
                    key.push_str(&function.arguments.join(", "));
                    key.push_str(")");
                    result.insert(key, function.expression.serialize(&context));
                }
                _ => {
                    result.insert(key, value.serialize(&context));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_statement() {
        let host_api = Rc::new(HostApi::builder().build());
        let session = Session::new(host_api.clone());
        let settings = Settings::default();
        assert_eq!(
            session.push("1+2", &settings),
            Session {
                host_api,
                stack: Stack::new(),
                statements: vec![Statement {
                    input: "1+2".into(),
                    output: Ok("3".into()),
                    stack: Stack::new(),
                }]
            }
        )
    }

    #[test]
    fn define_symbol_and_evaluate_later() {
        let host_api = Rc::new(HostApi::builder().build());
        let session = Session::new(host_api.clone());
        let settings = Settings::default();
        let mut stack_after_definition = Stack::new();
        stack_after_definition.insert("a", Number::new(2.0).into());
        assert_eq!(
            session.push("a := 2", &settings).push("a", &settings),
            Session {
                host_api,
                stack: stack_after_definition.clone().into(),
                statements: vec![
                    Statement {
                        input: "a := 2".into(),
                        output: Ok("a := 2".into()),
                        stack: Stack::new(),
                    },
                    Statement {
                        input: "a".into(),
                        output: Ok("2".into()),
                        stack: stack_after_definition.into(),
                    }
                ]
            }
        )
    }
}

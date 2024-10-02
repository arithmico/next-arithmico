use ast::{Function, Node};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Function {
    fn evaluate(&self, _context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_function") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "function",
            )));
        }

        Ok(Node::Function(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use ast::Symbol;

    use super::*;

    #[test]
    fn evaluate_function() {
        let context = Context::default();
        let result = Function::new(vec![String::from("x")], Symbol::new("x"))
            .evaluate(&context)
            .unwrap();
        assert_eq!(
            result,
            Function::new(vec![String::from("x")], Symbol::new("x"))
        );
    }
}

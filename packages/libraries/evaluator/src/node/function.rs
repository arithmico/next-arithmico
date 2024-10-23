use ast::{Function, Node};
use common::{EvaluateNodeContext, EvaluateNodeError};

use crate::evaluate::EvaluateNode;

impl EvaluateNode for Function {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_function") {
            return Err(EvaluateNodeError::unsupported_datatype("Function"));
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
        let context = EvaluateNodeContext::default();
        let result = Function::new(vec![String::from("x")], Symbol::new("x"))
            .evaluate(&context)
            .unwrap();
        assert_eq!(
            result,
            Function::new(vec![String::from("x")], Symbol::new("x"))
        );
    }
}

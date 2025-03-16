use crate::core::{
    EvaluateNode, EvaluateNodeContext, EvaluateNodeError, Function,
    GetNodeType, Node,
};

impl EvaluateNode for Function {
    fn evaluate(
        &self,
        _context: &EvaluateNodeContext,
    ) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_function") {
            return Err(EvaluateNodeError::unsupported_datatype(
                self.node_type(),
            ));
        }

        Ok(Node::Function(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{FunctionSignature, NodeType, Symbol};

    use super::*;

    #[test]
    fn evaluate_function() {
        let context = EvaluateNodeContext::default();
        let result = Function::new(
            FunctionSignature::new()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Symbol::new("x"),
        )
        .evaluate(&context)
        .unwrap();
        assert_eq!(
            result,
            Function::new(
                FunctionSignature::new()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x")
            )
        );
    }
}

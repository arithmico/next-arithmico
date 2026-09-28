use node::{Function, GetNodeType, Node};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for Function {
    fn evaluate(&self, _context: Options) -> Result<Node, Error> {
        if !cfg!(feature = "datatype_function") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        Ok(Node::Function(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use node::{FunctionSignature, NodeType, Symbol};

    use crate::{Api, Stack};

    use super::*;

    #[test]
    fn evaluate_function() {
        let stack = Stack::new();
        let api = Api::default();
        let options = Options::new(&stack, &api, common::AngleUnit::Radian);
        let result = Function::new(
            FunctionSignature::default()
                .argument("x", |argument| argument.node_type(NodeType::Any))
                .add_return_type(NodeType::Any),
            Symbol::new("x"),
        )
        .evaluate(options)
        .unwrap();
        assert_eq!(
            result,
            Function::new(
                FunctionSignature::default()
                    .argument("x", |argument| argument.node_type(NodeType::Any))
                    .add_return_type(NodeType::Any),
                Symbol::new("x")
            )
        );
    }
}

use ast::{Node, Tensor};

use crate::{evaluate::EvaluateNode, Context, EvaluateNodeError};

impl EvaluateNode for Tensor {
    fn evaluate(&self, context: &Context) -> Result<Node, EvaluateNodeError> {
        if !cfg!(feature = "datatype_tensor") {
            return Err(EvaluateNodeError::UnsupportedDataType(String::from(
                "tensor",
            )));
        }

        Ok(Tensor::new(
            self.elements
                .iter()
                .map(|element| element.evaluate(context))
                .collect::<Result<Vec<_>, EvaluateNodeError>>()?,
        ))
    }
}

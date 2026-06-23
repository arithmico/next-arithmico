use node::FunctionSignature;

use crate::{core::EvaluateNodeError, ArgumentMapping};

pub trait FunctionArguments<'a>: Sized {
    fn from_mapping(
        argument_mapping: &'a ArgumentMapping,
    ) -> Result<Self, EvaluateNodeError>;

    fn signature() -> FunctionSignature;
}

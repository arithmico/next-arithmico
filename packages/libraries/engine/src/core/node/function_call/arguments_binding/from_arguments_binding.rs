use crate::{ArgumentsBinding, core::EvaluateNodeError};

pub trait FromArgumentsBinding: Sized {
    fn from_arguments_binding(
        arguments: &ArgumentsBinding,
    ) -> Result<Self, EvaluateNodeError>;
}

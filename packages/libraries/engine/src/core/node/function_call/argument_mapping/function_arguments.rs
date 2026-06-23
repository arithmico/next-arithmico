use node::FunctionSignature;

use crate::{
    core::{EvaluateNodeError, TranslatedString},
    ArgumentMapping,
};

pub trait FunctionArguments<'a>: Sized {
    fn from_mapping(
        argument_mapping: &'a ArgumentMapping,
    ) -> Result<Self, EvaluateNodeError>;

    fn signature() -> FunctionSignature;

    fn function_description() -> TranslatedString;

    fn function_name() -> &'static str;
}

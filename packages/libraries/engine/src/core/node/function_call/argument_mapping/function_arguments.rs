use evaluator::Error;
use node::FunctionSignature;

use crate::{ArgumentMapping, core::TranslatedString};

pub trait FunctionArguments<'a>: Sized {
    fn from_mapping(
        argument_mapping: &'a ArgumentMapping,
    ) -> Result<Self, Error>;

    fn signature() -> FunctionSignature;

    fn function_description() -> TranslatedString;

    fn function_name() -> &'static str;
}

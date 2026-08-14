use node::FunctionSignature;
use translate_core::RenderedTranslatedMessage;

use crate::{ArgumentMapping, Error};

pub trait FunctionArguments<'a>: Sized {
    fn from_mapping(
        argument_mapping: &'a ArgumentMapping,
    ) -> Result<Self, Error>;

    fn signature() -> FunctionSignature;

    fn function_description() -> RenderedTranslatedMessage;

    fn function_name() -> &'static str;
}

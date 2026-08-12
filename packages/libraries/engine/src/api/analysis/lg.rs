use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("lg")]
#[description(
    Language::German,
    "Berechnet den dekadischen Logarithmus von x (zur Basis 10)."
)]
#[description(
    Language::English,
    "Calculates the common logarithm of x (base 10)."
)]
pub struct LgArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct LgEndpoint;

impl FunctionEndpoint for LgEndpoint {
    type Output = Number;

    type Arguments<'a> = LgArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LgArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        x.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = x.value.log10();

        Ok(Number::new(result))
    }
}

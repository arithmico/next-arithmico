use engine_derive::FunctionArguments;
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("log")]
#[description(
    Language::German,
    "Berechnet den Logarithmus von x zur Basis base."
)]
#[description(
    Language::English,
    "Calculates the logarithm of x to the given base."
)]
pub struct LogArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,

    #[description(Language::German, "Basis")]
    #[description(Language::English, "base")]
    base: &'a Number,
}

pub struct LogEndpoint;

impl FunctionEndpoint for LogEndpoint {
    type Output = Number;

    type Arguments<'a> = LogArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LogArgs { x, base }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        x.validate_greater_than(0.0)?;
        base.validate_greater_than(0.0)?.validate_not_equal(1.0)?;

        let result = x.value.log(base.value);

        Ok(Number::new(result))
    }
}

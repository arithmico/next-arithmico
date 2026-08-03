use engine_derive::FunctionArguments;
use node::Number;
use trace::{Tracable, TracableMut};

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
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
    ) -> Result<Self::Output, EvaluateNodeError> {
        if x.value <= 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter.must_be_greater_than",
            )
            .key("value", 0.to_string())
            .build()
            .with_optional_span(x.hull()));
        }

        let result = x.value.log10();

        Ok(Number::new(result))
    }
}

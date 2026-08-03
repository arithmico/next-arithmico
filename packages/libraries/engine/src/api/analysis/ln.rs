use engine_derive::FunctionArguments;
use node::Number;
use trace::{Tracable, TracableMut};

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("ln")]
#[description(
    Language::German,
    "Berechnet den natuerlichen Logarithmus von x (zur Basis e)."
)]
#[description(
    Language::English,
    "Calculates the natural logarithm of x (base e)."
)]
pub struct LnArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct LnEndpoint;

impl FunctionEndpoint for LnEndpoint {
    type Output = Number;

    type Arguments<'a> = LnArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LnArgs { x }: Self::Arguments<'a>,
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

        let result = x.value.ln();

        Ok(Number::new(result))
    }
}

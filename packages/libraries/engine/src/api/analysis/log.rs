use engine_derive::FunctionArguments;
use node::Number;
use trace::{Tracable, TracableMut};

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
        if x.value <= 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter.must_be_greater_than",
            )
            .key("value", 0.to_string())
            .build()
            .with_optional_span(x.hull()));
        }

        if base.value <= 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter.must_be_greater_than",
            )
            .key("value", 0.to_string())
            .build()
            .with_optional_span(base.hull()));
        }

        if base.value == 1.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter.must_be_not_equal",
            )
            .key("value", 1.to_string())
            .build()
            .with_optional_span(base.hull()));
        }

        let result = x.value.log(base.value);

        Ok(Number::new(result))
    }
}

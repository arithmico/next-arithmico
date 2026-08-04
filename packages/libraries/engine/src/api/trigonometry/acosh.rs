use engine_derive::FunctionArguments;
use node::Number;
use trace::{Tracable, TracableMut};

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("acosh")]
#[description(
    Language::German,
    "Berechnet den Area Cosinus hyperbolicus (die Umkehrfunktion des Cosinus hyperbolicus) von x."
)]
#[description(
    Language::English,
    "Calculates the inverse hyperbolic cosine of x."
)]
pub struct AcoshArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AcoshEndpoint;

impl FunctionEndpoint for AcoshEndpoint {
    type Output = Number;

    type Arguments<'a> = AcoshArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AcoshArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        if value < 1.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter.must_be_greater_than_or_equal",
            )
                .key("value", 1.to_string())
                .build()
                .with_optional_span(x.hull())
            );
        }

        Ok(Number::new(value.acosh()))
    }
}

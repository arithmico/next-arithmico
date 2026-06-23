use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

#[derive(FunctionArguments)]
#[name("asin")]
#[description(Language::German, "Berechnet den Arkussinus von x.")]
#[description(Language::English, "Calculates the arcsine of x.")]
pub struct AsinArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AsinEndpoint;

impl FunctionEndpoint for AsinEndpoint {
    type Output = Number;

    type Arguments<'a> = AsinArgs<'a>;

    // TODO: unit tests
    // TODO: handle special values
    fn executor<'a>(
        AsinArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        if value < -1.0 || value > 1.0 || !value.is_finite() {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.trigonometry.asin.out-of-bounds",
            )
            .build()
            .with_tracable(x));
        }

        let result = value.asin();

        Ok(Number::new(result))
    }
}

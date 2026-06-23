use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

#[derive(FunctionArguments)]
#[name("acos")]
#[description(Language::German, "Berechnet den Arkuscosinus von x.")]
#[description(Language::English, "Calculates the arc cosine of x.")]
pub struct AcosArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AcosEndpoint;

impl FunctionEndpoint for AcosEndpoint {
    type Output = Number;

    type Arguments<'a> = AcosArgs<'a>;

    // TODO: unit tests
    // TODO: handle special values
    fn executor<'a>(
        AcosArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        if value < -1.0 || value > 1.0 || !value.is_finite() {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.trigonometry.acos.out-of-bounds",
            )
            .build()
            .with_tracable(x));
        }

        let result = value.acos();

        Ok(Number::new(result))
    }
}

use std::f64::consts::PI;

use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use node::Number;
use translate_core::Language;

use crate::core::FunctionEndpoint;

#[derive(FunctionArguments)]
#[name("tan")]
#[description(Language::German, "Berechnet den Tangens von x.")]
#[description(Language::English, "Calculates the tangent of x.")]
pub struct TanArgs<'a> {
    #[description(Language::German, "Winkel")]
    #[description(Language::English, "angle")]
    x: &'a Number,
}

pub struct TanEndpoint;

impl FunctionEndpoint for TanEndpoint {
    type Output = Number;
    type Arguments<'a> = TanArgs<'a>;

    fn executor<'a>(
        TanArgs { x }: Self::Arguments<'a>,
        _context: &crate::Context,
    ) -> Result<Number, crate::core::EvaluateNodeError> {
        let value = x.value;

        if value.is_close_to_multiple_of(PI) {
            return Ok(Number::new(0.));
        }

        Ok(Number::new(value.tan()))
    }
}

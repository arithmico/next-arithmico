use std::f64::consts::PI;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use float_utils::F64Extension;
use node::Number;

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

    // TODO: unit tests
    fn executor<'a>(
        TanArgs { x }: Self::Arguments<'a>,
        context: Options,
    ) -> Result<Number, Error> {
        let value = context.angle_unit.to_radians(x.value);

        if value.is_close_to_multiple_of(PI) {
            return Ok(Number::new(0.));
        }

        Ok(Number::new(value.tan()))
    }
}

use std::f64::consts::PI;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use float_utils::F64Extension;
use node::Number;

#[derive(FunctionArguments)]
#[name("sin")]
#[description(Language::German, "Berechnet den Sinus von x.")]
#[description(Language::English, "Calculate the sine of x.")]
pub struct SinArgs<'a> {
    #[description(Language::German, "Winkel")]
    #[description(Language::English, "angle")]
    x: &'a Number,
}

pub struct SinEndpoint;

impl FunctionEndpoint for SinEndpoint {
    type Output = Number;

    type Arguments<'a> = SinArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SinArgs { x }: Self::Arguments<'a>,
        context: Options,
    ) -> Result<Self::Output, Error> {
        let value = context.angle_unit.to_radians(x.value);

        if value.is_close_to_multiple_of(PI) {
            return Ok(Number::new(0.0));
        }

        Ok(Number::new(value.sin()))
    }
}

use std::f64::consts::PI;

use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

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
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        if value.is_close_to_multiple_of(PI) {
            return Ok(Number::new(0.0));
        }

        Ok(Number::new(value.sin()))
    }
}

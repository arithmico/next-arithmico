use std::f64::consts::PI;

use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

#[derive(FunctionArguments)]
#[name("cos")]
#[description(Language::German, "Berechnet den Cosinus von x.")]
#[description(Language::English, "Calculate the consine of x.")]
pub struct CosArgs<'a> {
    #[description(Language::German, "Winkel")]
    #[description(Language::English, "angle")]
    x: &'a Number,
}

pub struct CosEndpoint;

impl FunctionEndpoint for CosEndpoint {
    type Output = Number;

    type Arguments<'a> = CosArgs<'a>;

    fn executor<'a>(
        CosArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        if value.is_close_to_multiple_of(2.0 * PI) {
            return Ok(Number::new(1.));
        } else if value.is_close_to_shifted_multiple_of(PI, PI / 2.0) {
            return Ok(Number::new(0.));
        } else if value.is_close_to_shifted_multiple_of(2.0 * PI, PI) {
            return Ok(Number::new(-1.));
        }

        Ok(Number::new(value.cos()))
    }
}

use std::{collections::HashMap, f64::consts::PI};
use translate_core::Language;

use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use node::Number;

use crate::core::FunctionEndpoint;

// TODO: add descriptions with attribute macros

#[derive(FunctionArguments)]
#[description(Language::German, "Tangens")]
#[description(Language::English, "tangent")]
pub struct TanArgs<'a> {
    #[description(Language::German, "Winkel")]
    #[description(Language::Enlish, "Angle")]
    x: &'a Number,
}

pub struct TanEndpoint;

impl FunctionEndpoint for TanEndpoint {
    type Output = Number;
    type Arguments<'a> = TanArgs<'a>;

    fn name() -> &'static str {
        "tan"
    }

    fn executor<'a>(
        TanArgs { x }: Self::Arguments<'a>,
        _context: &crate::Context,
    ) -> Result<Number, crate::core::EvaluateNodeError> {
        let value = x.value;

        if value.is_close_to_multiple_of(PI) {
            return Ok(Number::new_self(0.));
        }

        Ok(Number::new_self(value.tan()))
    }
}

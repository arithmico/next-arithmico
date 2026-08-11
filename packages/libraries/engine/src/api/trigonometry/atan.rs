use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("atan")]
#[description(Language::German, "Berechnet den Arkustangens von x.")]
#[description(Language::English, "Calculates the arc tangent of x.")]
pub struct AtanArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AtanEndpoint;

impl FunctionEndpoint for AtanEndpoint {
    type Output = Number;

    type Arguments<'a> = AtanArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AtanArgs { x }: Self::Arguments<'a>,
        context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;
        let result = context.angle_unit.from_radians(value.atan());
        Ok(Number::new(result))
    }
}

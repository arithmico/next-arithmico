use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
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

    // TODO: handle special values
    fn executor<'a>(
        AtanArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;
        let result = value.atan();
        Ok(Number::new(result))
    }
}

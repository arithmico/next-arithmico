use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("cosh")]
#[description(Language::German, "Berechnet den Cosinus hyperbolicus von x.")]
#[description(Language::English, "Calculates the hyperbolic cosine of x.")]
pub struct CoshArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct CoshEndpoint;

impl FunctionEndpoint for CoshEndpoint {
    type Output = Number;

    type Arguments<'a> = CoshArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CoshArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        Ok(Number::new(value.cosh()))
    }
}

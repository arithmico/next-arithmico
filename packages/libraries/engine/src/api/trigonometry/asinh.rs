use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("asinh")]
#[description(
    Language::German,
    "Berechnet den Area Sinus hyperbolicus (die Umkehrfunktion des Sinus hyperbolicus) von x."
)]
#[description(
    Language::English,
    "Calculates the inverse hyperbolic sine of x."
)]
pub struct AsinhArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AsinhEndpoint;

impl FunctionEndpoint for AsinhEndpoint {
    type Output = Number;

    type Arguments<'a> = AsinhArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AsinhArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        Ok(Number::new(value.asinh()))
    }
}

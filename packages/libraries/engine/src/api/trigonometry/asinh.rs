use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

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
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let value = x.value;

        Ok(Number::new(value.asinh()))
    }
}

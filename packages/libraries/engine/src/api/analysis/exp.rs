use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("exp")]
#[description(
    Language::German,
    "Berechnet die Potenz der eulerschen Zahl e mit dem Exponenten x (e^x)."
)]
#[description(
    Language::English,
    "Calculates the power of Euler's number e to the exponent x (e^x)."
)]
pub struct ExpArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct ExpEndpoint;

impl FunctionEndpoint for ExpEndpoint {
    type Output = Number;

    type Arguments<'a> = ExpArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        ExpArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let result = x.value.exp();

        Ok(Number::new(result))
    }
}

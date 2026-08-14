use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("tanh")]
#[description(Language::German, "Berechnet den Tangens hyperbolicus von x.")]
#[description(Language::English, "Calculates the hyperbolic tangent of x.")]
pub struct TanhArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct TanhEndpoint;

impl FunctionEndpoint for TanhEndpoint {
    type Output = Number;

    type Arguments<'a> = TanhArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        TanhArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let value = x.value;

        Ok(Number::new(value.tanh()))
    }
}

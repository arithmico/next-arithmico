use engine_derive::FunctionArguments;
use evaluator::Error;
use node::Number;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("sinh")]
#[description(Language::German, "Berechnet den Sinus hyperbolicus von x.")]
#[description(Language::English, "Calculates the hyperbolic sine of x.")]
pub struct SinhArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct SinhEndpoint;

impl FunctionEndpoint for SinhEndpoint {
    type Output = Number;

    type Arguments<'a> = SinhArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SinhArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        let value = x.value;

        Ok(Number::new(value.sinh()))
    }
}

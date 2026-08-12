use engine_derive::FunctionArguments;
use evaluator::Error;
use node::Number;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("abs")]
#[description(Language::German, "Berechnet den absoluten Betrag von x.")]
#[description(Language::English, "Calculates the absolute value of x.")]
pub struct AbsArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AbsEndpoint;

impl FunctionEndpoint for AbsEndpoint {
    type Output = Number;

    type Arguments<'a> = AbsArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AbsArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        let result = x.value.abs();

        Ok(Number::new(result))
    }
}

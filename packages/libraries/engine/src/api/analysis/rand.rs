use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::random_float;
use node::Number;

#[derive(FunctionArguments)]
#[name("rand")]
#[description(
    Language::German,
    "Gibt eine zufällige Zahl zwischen 0 (einschließlich) und 1 (ausschließlich) zurück."
)]
#[description(
    Language::English,
    "Returns a random number between 0 (inclusive) and 1 (exclusive)."
)]
pub struct RandArgs;

pub struct RandEndpoint;

impl FunctionEndpoint for RandEndpoint {
    type Output = Number;
    type Arguments<'a> = RandArgs;

    // TODO: unit tests
    fn executor<'a>(
        RandArgs {}: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        Ok(Number::new(random_float()))
    }
}

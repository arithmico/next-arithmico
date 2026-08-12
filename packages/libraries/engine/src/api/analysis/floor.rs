use engine_derive::FunctionArguments;
use evaluator::Error;
use node::Number;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("floor")]
#[description(
    Language::German,
    "Rundet x auf die naechstkleinere oder gleiche ganze Zahl ab."
)]
#[description(
    Language::English,
    "Rounds x down to the nearest integer less than or equal to x."
)]
pub struct FloorArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct FloorEndpoint;

impl FunctionEndpoint for FloorEndpoint {
    type Output = Number;

    type Arguments<'a> = FloorArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        FloorArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        let result = x.value.floor();

        Ok(Number::new(result))
    }
}

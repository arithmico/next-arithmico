use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("ceil")]
#[description(
    Language::German,
    "Rundet x auf die naechstgroessere oder gleiche ganze Zahl auf."
)]
#[description(
    Language::English,
    "Rounds x up to the nearest integer greater than or equal to x."
)]
pub struct CeilArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct CeilEndpoint;

impl FunctionEndpoint for CeilEndpoint {
    type Output = Number;

    type Arguments<'a> = CeilArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CeilArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let result = x.value.ceil();

        Ok(Number::new(result))
    }
}

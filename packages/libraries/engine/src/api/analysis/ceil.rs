use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

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
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let result = x.value.ceil();

        Ok(Number::new(result))
    }
}

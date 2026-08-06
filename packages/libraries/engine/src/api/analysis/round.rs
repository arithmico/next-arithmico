use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("round")]
#[description(Language::German, "Rundet x auf die naechste ganze Zahl.")]
#[description(Language::English, "Rounds x to the nearest integer.")]
pub struct RoundArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct RoundEndpoint;

impl FunctionEndpoint for RoundEndpoint {
    type Output = Number;

    type Arguments<'a> = RoundArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        RoundArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let result = x.value.round();

        Ok(Number::new(result))
    }
}

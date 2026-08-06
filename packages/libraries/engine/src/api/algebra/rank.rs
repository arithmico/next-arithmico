use engine_derive::FunctionArguments;
use node::{Number, Tensor};

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("tensor:rank")]
#[description(Language::German, "Berechnet den Rang eines Tensors.")]
#[description(Language::English, "Calculates the rank of a tensor.")]
pub struct RankArgs<'a> {
    #[description(Language::German, "Tensor")]
    #[description(Language::English, "tensor")]
    x: &'a Tensor,
}

pub struct RankEndpoint;

impl FunctionEndpoint for RankEndpoint {
    type Output = Number;

    type Arguments<'a> = RankArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        RankArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        Ok(Number::new(x.get_rank() as f64))
    }
}

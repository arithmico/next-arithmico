use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::{DowncastNodeVec, Number, Tensor};
use node_validator::TensorValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("length")]
#[description(
    Language::German,
    "Berechnet die Länge (den Betrag) des Vektors."
)]
#[description(
    Language::English,
    "Calculates the length (magnitude) of the vector."
)]
pub struct LengthArgs<'a> {
    #[description(Language::German, "Vektor")]
    #[description(Language::English, "vector")]
    x: &'a Tensor,
}

pub struct LengthEndpoint;

impl FunctionEndpoint for LengthEndpoint {
    type Output = Number;

    type Arguments<'a> = LengthArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LengthArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        x.validate_not_empty()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_rank(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let value = x
            .elements
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .iter()
            .fold(0.0, |acc, v| acc + v.value.powi(2))
            .sqrt();

        Ok(Number::new(value))
    }
}

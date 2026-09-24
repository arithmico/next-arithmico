use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::{DowncastNodeVec, Number, Tensor};
use validator::TensorValidator;

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
        _context: Options,
    ) -> Result<Self::Output, Error> {
        x.validate_not_empty()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_rank(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let value = x
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .iter()
            .fold(0.0, |acc, v| acc + v.value.powi(2))
            .sqrt();

        Ok(Number::new(value))
    }
}

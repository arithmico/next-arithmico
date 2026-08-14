use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use node_validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("acosh")]
#[description(
    Language::German,
    "Berechnet den Area Cosinus hyperbolicus (die Umkehrfunktion des Cosinus hyperbolicus) von x."
)]
#[description(
    Language::English,
    "Calculates the inverse hyperbolic cosine of x."
)]
pub struct AcoshArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AcoshEndpoint;

impl FunctionEndpoint for AcoshEndpoint {
    type Output = Number;

    type Arguments<'a> = AcoshArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AcoshArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        x.validate_greater_than_or_equal(1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let value = x.value;

        Ok(Number::new(value.acosh()))
    }
}

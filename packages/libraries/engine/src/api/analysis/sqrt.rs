use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use node_validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("sqrt")]
#[description(Language::German, "Berechnet die Quadratwurzel von x.")]
#[description(Language::English, "Calculates the square root of x.")]
pub struct SqrtArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct SqrtEndpoint;

impl FunctionEndpoint for SqrtEndpoint {
    type Output = Number;

    type Arguments<'a> = SqrtArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SqrtArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        x.validate_greater_than_or_equal(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = x.value.sqrt();

        Ok(Number::new(result))
    }
}

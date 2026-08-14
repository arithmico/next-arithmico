use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use node_validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("ln")]
#[description(
    Language::German,
    "Berechnet den natuerlichen Logarithmus von x (zur Basis e)."
)]
#[description(
    Language::English,
    "Calculates the natural logarithm of x (base e)."
)]
pub struct LnArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct LnEndpoint;

impl FunctionEndpoint for LnEndpoint {
    type Output = Number;

    type Arguments<'a> = LnArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LnArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        x.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = x.value.ln();

        Ok(Number::new(result))
    }
}
